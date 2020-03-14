mod config;
mod error;
mod macros;
mod client;
mod logger;

use config::Config;
use error::BariumResult;
use client::{Client, Clients};
use std::{
    env,
    time::Duration,
    net::{Shutdown, TcpStream, TcpListener},
    io::{Read, Write},
    collections::HashMap,
    sync::{Arc, RwLock, mpsc}
};
use padlock;
use bincode;
use barium_shared::{AfkStatus, ToClient, ToServer, hash::sha3_256};
use log::{debug, info, warn};
use lazy_static::lazy_static;
use native_tls::{Identity, TlsAcceptor, TlsStream};

lazy_static! {
    static ref CONF: Config = Config::load(env::args().nth(1)).unwrap_or_default();
}

async fn handle_client(mut stream: TlsStream<TcpStream>, clients: Clients) -> BariumResult<()> {

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
                let _ = stream.shutdown();
                break;
            },

            Ok(len) => {

                match bincode::deserialize::<ToServer>(&buf[0..len]) {

                    Ok(data) => {

                        match data {

                            ToServer::Ping => {

                                let pong = ToClient::Pong;
                                let data = bincode::serialize(&pong)?;

                                stream.write_all(&data[..])?;

                            },

                            ToServer::Hello(sender, user_public_key, password) => {

                                if password != CONF.server.password {
                                    drop(stream.shutdown());
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

                                    let _ = stream.shutdown();
                                    break;

                                } else {

                                    client_hash = Some(hash);

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

                                            let _ = user.send_data(ToClient::FriendsOnline(friends_online));

                                        },

                                        // Sender not authenticated
                                        None => {} // TODO: Drop connection

                                    }

                                });

                            },

                            ToServer::Message(sender, message) => {

                                let hash = sha3_256(&sender);

                                padlock::rw_read_lock(&clients, |lock| {

                                    match lock.get(&hash) {

                                        Some(_) => {

                                            match lock.get(&message.to) {

                                                Some(client) => {
                                                    let _ = client.send_data(ToClient::Message(message.data));
                                                },

                                                // Recipient is not connected
                                                None => {} // Do nothing

                                            }

                                        },

                                        // Sender not authenticated
                                        None => {} // TODO: Drop connection

                                    }

                                });

                                // let exists = padlock::rw_read_lock(&clients, |lock| {
                                //     lock.get(&hash).is_some()
                                // });

                                // if exists {

                                //     padlock::rw_read_lock(&clients, |lock| {
                                //         match lock.get(&message.to) {
                                //             Some(user) => user.send_data(ToClient::Message(message.data)),
                                //             None => Ok(())
                                //         }
                                //     })?;

                                // }

                            }

                        }

                    },

                    Err(e) => {
                        // Recv invalid data
                        warn!("{}", e);
                        stream.shutdown()?;
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
                stream.shutdown()?;
                break;
            }

        } // end of match read

        debug!("{:#?}", clients);

        tokio::time::delay_for(Duration::from_millis(100u64)).await;

    } // end of loop

    debug!("Connection closed");

    Ok(())

}

#[tokio::main]
async fn main() -> BariumResult<()> {

    logger::configure()?;

    info!("Starting Barium Server...");

    // Log config
    CONF.log();

    // Get cert
    let cert_bytes = std::fs::read(&CONF.cert.path)?;
    let cert_pkcs12 = Identity::from_pkcs12(&cert_bytes, CONF.cert.password.as_str())?;
    let tls_acceptor = Arc::new(TlsAcceptor::new(cert_pkcs12)?);

    // Listener variables
    let addr = CONF.server.address.parse::<std::net::IpAddr>()?;
    let port = CONF.server.port;

    // Listener
    let listener = TcpListener::bind((addr, port))?;

    // Keep track of clients
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    loop {

        let (stream, remote_addr) = listener.accept()?;

        // Block blacklisted ips. Drop incoming connections.
        debug!("New connection from {}", remote_addr);

        let addr = remote_addr.ip().to_string();
        if CONF.blacklist.contains(&addr) {
            warn!("Blacklist dropping connection from {}", remote_addr);
            drop(stream.shutdown(Shutdown::Both));
            drop(stream);
            drop(remote_addr);
            continue;
        }

        let clients_clone = Arc::clone(&clients);
        let acceptor_clone = Arc::clone(&tls_acceptor);
        tokio::spawn(async move {

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
