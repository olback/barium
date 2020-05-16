mod config;
mod error;
mod macros;
mod client;
mod logger;
mod tokio_runtime_builder;
mod utils;

use tokio;
use config::Config;
use error::BariumResult;
use client::{Client, Clients};
use std::{
    env,
    time::Duration,
    net::{Shutdown, TcpStream, TcpListener},
    io::{Read, Write},
    collections::HashMap,
    sync::{Arc, RwLock, mpsc, atomic::{AtomicU16, Ordering}}
};
use padlock;
use barium_shared::{AfkStatus, ToClient, ToServer, ServerProperties, ToHex, hash::sha3_256};
use log::{debug, info, warn};
use lazy_static::lazy_static;
use native_tls::{Identity, TlsAcceptor, TlsStream};
use tokio_runtime_builder::TokioRuntimeBuilder;

lazy_static! {
    static ref CONF: Config = Config::load(env::args().nth(1).expect("specify config file as the first argument")).unwrap();
    static ref CLIENT_COUNT: AtomicU16 = AtomicU16::new(0);
    static ref PROPERTIES: ServerProperties = utils::get_server_properties(&CONF);
}

async fn handle_client(mut stream: TlsStream<TcpStream>, clients: Clients) -> BariumResult<()> {

    CLIENT_COUNT.fetch_add(1, Ordering::SeqCst);

    let mut buf = [0u8; 8192];
    let mut client_hash: Option<[u8; 32]> = None;

    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    loop {

        match rx.try_recv() {

            Ok(data) => {
                stream.write(&data[..])?;
            },
            Err(_) => {}

        };

        match stream.read(&mut buf[..]) {

            Ok(len) if len == 0 => {
                match client_hash {
                    Some(ref ch) => padlock::rw_write_lock(&clients, |lock| {
                        lock.remove(ch);
                    }),
                    None => ()
                }
                break;
            },

            Ok(len) => {

                debug!("{}:{:?}", len, &buf[0..len]);

                match rmp_serde::from_read_ref::<_, ToServer>(&buf[0..len]) {

                    Ok(data) => {

                        match data {

                            ToServer::Ping => {

                                let pong = ToClient::Pong;
                                let data = rmp_serde::to_vec(&pong)?;

                                stream.write_all(&data[..])?;

                            },

                            ToServer::GetProperties => {

                                let props = ToClient::Properties(PROPERTIES.clone());
                                let data = rmp_serde::to_vec(&props)?;

                                stream.write_all(&data[..])?;

                            },

                            ToServer::GetPublicKey(sender, user) => {

                                let hash = sha3_256(&sender);

                                let authenticated = padlock::rw_read_lock(&clients, |lock| {
                                    lock.get(&hash).is_some()
                                });

                                if authenticated {

                                    let pubkey = padlock::rw_read_lock(&clients, |lock| {
                                        match lock.get(&user) {
                                            Some(u) => Some(u.get_public_key()),
                                            None => None
                                        }
                                    });

                                    match pubkey {

                                        Some(key) => {
                                            let pubkey = ToClient::PublicKey(user, key);
                                            let mut data = rmp_serde::to_vec(&pubkey)?;
                                            stream.write_all(&mut data[..])?;
                                        },

                                        None => {} // TODO: What happens here?

                                    }

                                } else {

                                    // Sender not authenticated
                                    // TODO: Drop connection

                                }

                            },

                            ToServer::Hello(sender, user_public_key, password) => {

                                if password != CONF.server.password {
                                    break;
                                }

                                let hash = sha3_256(&sender);

                                let exists = padlock::rw_read_lock(&clients, |lock| {
                                    lock.get(&hash).is_some()
                                });

                                if exists {

                                    match stream.get_ref().peer_addr() {
                                        Ok(addr) => {
                                            warn!("User from {} tried to recreate a session!", addr);
                                        },
                                        Err(_) => ()
                                    };

                                    break;

                                } else {

                                    client_hash = Some(hash);

                                    debug!("New client: hash:{} id:{}", client_hash.unwrap().to_hex(), sender.to_hex());

                                    let new_client = Client::new(&tx, user_public_key, AfkStatus::Away(None));

                                    padlock::rw_write_lock(&clients, |lock| {
                                        lock.insert(hash, new_client);
                                    });

                                }

                            },

                            ToServer::KeepAlive(sender, friends, status) => {

                                let hash = sha3_256(&sender);

                                padlock::rw_read_lock(&clients, |lock| {

                                    match lock.get(&hash) {

                                        Some(user) => {

                                            user.set_idle(status);

                                            let mut friends_online = Vec::<([u8; 32], AfkStatus)>::new();
                                            for friend in &friends {
                                                match lock.get(friend) {
                                                    Some(fc) => friends_online.push((friend.clone(), fc.get_idle())),
                                                    None => ()
                                                }
                                            }

                                            // TODO: Use stream directly to avoid aquiring a mutex lock
                                            let _ = user.send_data(ToClient::FriendsOnline(friends_online));

                                        },

                                        // Sender not authenticated
                                        None => {} // TODO: Drop connection

                                    }

                                });

                            },

                            ToServer::Message(sender, reciever, message) => {

                                let hash = sha3_256(&sender);

                                padlock::rw_read_lock(&clients, |lock| {

                                    match lock.get(&hash) {

                                        Some(_) => {

                                            match lock.get(&reciever) {

                                                Some(client) => {
                                                    let _ = client.send_data(ToClient::Message(message));
                                                },

                                                // Recipient is not connected
                                                None => {} // TODO: Respond with error

                                            }

                                        },

                                        // Sender not authenticated
                                        None => {} // TODO: Drop connection

                                    }

                                });

                            }

                        }

                    },

                    Err(e) => {
                        // Recv invalid data
                        warn!("{}", e);
                        break;
                    }

                }

            },

            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}, // Do nothing

            Err(e) => {
                warn!("{:#?}", e);
                match client_hash {
                    Some(ref ch) => padlock::rw_write_lock(&clients, |lock| {
                        lock.remove(ch);
                    }),
                    None => ()
                }
                break;
            }

        } // end of match read

        tokio::time::delay_for(Duration::from_millis(100u64)).await;

    } // end of loop

    CLIENT_COUNT.fetch_sub(1, Ordering::SeqCst);

    stream.shutdown()?;

    if client_hash.is_some() {
        debug!("Connection closed");
    } else {
        debug!("Connection closed without authentication");
    }
    debug!("Total clients connected: {}", CLIENT_COUNT.load(Ordering::SeqCst));

    Ok(())

}

fn main() -> BariumResult<()> {

    // Configure logger
    logger::configure(CONF.log_level)?;

    // Create our own runtime
    let rt = TokioRuntimeBuilder::from_config(&CONF).build()?;

    info!("Starting Barium Server...");

    // Log config
    CONF.log();

    // Get cert
    let cert_bytes = std::fs::read(&CONF.cert.path)?;
    let cert_pkcs12 = Identity::from_pkcs12(&cert_bytes, CONF.cert.password.as_str())?;
    let mut tls_acceptor_builder = TlsAcceptor::builder(cert_pkcs12);
    tls_acceptor_builder.min_protocol_version(Some(native_tls::Protocol::Tlsv12));
    let tls_acceptor = Arc::new(tls_acceptor_builder.build()?);

    // Listener variables
    let addr = CONF.server.address.parse::<std::net::IpAddr>()?;
    let port = CONF.server.port;

    // Listener
    let listener = TcpListener::bind((addr, port))?;

    // Keep track of clients
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    loop {

        let (stream, remote_addr) = listener.accept()?;

        debug!("New connection from {}", remote_addr);

        // Block blacklisted ips. Drop incoming connections.
        let addr = remote_addr.ip();
        if CONF.is_blacklisted(&addr) {
            warn!("Blacklist dropping connection from {}", remote_addr);
            drop(stream.shutdown(Shutdown::Both));
            drop(stream);
            drop(remote_addr);
            continue;
        }

        let clients_clone = Arc::clone(&clients);
        let acceptor_clone = Arc::clone(&tls_acceptor);
        rt.spawn(async move {

            match acceptor_clone.accept(stream) {

                Ok(mut tls_stream) => {

                    tls_stream.get_mut().set_nonblocking(true).unwrap();
                    let _ = handle_client(tls_stream, clients_clone).await;

                },

                Err(_) => {}

            }

        });

    }

}
