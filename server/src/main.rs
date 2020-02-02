mod config;
mod error;
mod macros;
mod client;
mod utils;

use config::Config;
use error::BariumResult;
use client::{Client, Clients};

use std::{
    env,
    net::Shutdown,
    collections::HashMap,
    time::Duration,
    sync::{
        Arc,
        RwLock,
    }
};
use tokio::{
    sync::mpsc,
    net::TcpListener,
    time,
    io::{
        AsyncReadExt,
        AsyncWriteExt
    }
};
use padlock;
use bincode;
use barium_shared::{AfkStatus, ToClient, ToServer};
use tokio_io_timeout::TimeoutStream;

async fn handle_client(stream: tokio_tls::TlsStream<tokio::net::TcpStream>, clients: Clients) -> BariumResult<()> {

    let mut buf = [0u8; 8192];
    let mut client_id: Option<[u8; 32]> = None;

    let mut stream = TimeoutStream::new(stream);
    stream.set_read_timeout(Some(Duration::from_secs(2)));
    // stream.set_write_timeout(Some(Duration::from_secs(2)));

    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(64);

    loop {

        match rx.try_recv() {
            Ok(data) => {
                println!("Writing...?");
                stream.write_all(&data[..]).await?
            },
            Err(_) => ()
        }

        match stream.read(&mut buf[..]).await {

            Ok(len) if len == 0 => {
                match client_id {
                    Some(id) => {
                        println!("Closing stream to {:?}", id);
                        padlock::rw_write_lock(&clients, |lock| {
                            lock.remove(&id)
                        });
                        println!("{:#?}", &clients);
                    },
                    None => ()
                }
                stream.shutdown();
                return Ok(())
            },

            Ok(len) => {

                println!("Got {} bytes of data", len);

                match bincode::deserialize::<ToServer>(&buf[0..len]) {

                    Ok(data) => {

                        match data {

                            ToServer::KeepAlive(sender, friends, status) => {

                                let new_user = padlock::rw_read_lock(&clients, |lock| {

                                    match lock.get(&sender) {

                                        Some(client) => {

                                            client.set_idle(status);

                                            let mut friends_online = Vec::<([u8; 32], AfkStatus)>::new();
                                            for f in &friends {
                                                match lock.get(f) {
                                                    Some(fc) => {
                                                        if f == &fc.get_hash() {
                                                            friends_online.push((fc.get_hash(), fc.get_idle()))
                                                        }
                                                    },
                                                    None => {}
                                                }
                                            }

                                            let to_client = ToClient::FriendsOnline(friends_online);
                                            client.send_data(to_client);

                                            false
                                        },

                                        None => true // We cannot get a write lock here since we already have a read lock

                                    }

                                });

                                if new_user {

                                    client_id.replace(sender);

                                    let tx = tx.clone();

                                    padlock::rw_write_lock(&clients, |lock| {
                                        lock.insert(sender, Client::new(sender, status, tx));
                                    });

                                    // Aquire a new read-only lock to free the write lock as soon as possible
                                    padlock::rw_read_lock(&clients, |lock| {

                                        let mut friends_online = Vec::<([u8; 32], AfkStatus)>::new();
                                        for f in friends {
                                            match lock.get(&f) {
                                                Some(fc) => {
                                                    if f == fc.get_hash() {
                                                        friends_online.push((fc.get_hash(), fc.get_idle()))
                                                    }
                                                },
                                                None => {}
                                            }
                                        }
                                        let to_client = ToClient::FriendsOnline(friends_online);
                                        println!("Sending to client...");
                                        stream.write_all(&bincode::serialize(&to_client).unwrap()[..]);

                                    });

                                    println!("{:#?}", &clients);

                                }

                            },

                            ToServer::Message(message) => {

                                padlock::rw_read_lock(&clients, |lock| {

                                    match lock.get(&message.to) {

                                        Some(user) => {
                                            user.send_data(ToClient::Message(message.data))
                                        },

                                        None => {}

                                    }

                                });

                            }

                        }

                    },

                    Err(e) => {
                        eprintln!("{}", e)
                    }

                }

            },

            Err(ref e) => {

                if e.kind() != std::io::ErrorKind::TimedOut {
                    let err = new_err!(e);
                    eprintln!("{}", err);
                    return Err(err)
                }

            }

        } // end of match read

        time::delay_for(Duration::from_millis(500)).await;

    } // end of loop

}

#[tokio::main]
async fn main() -> BariumResult<()> {

    println!("Starting Barium Server...");

    // Load config
    let config = Config::load(env::args().nth(1))?;
    println!("{:#?}", config);

    // Listener variables
    let addr = config.server.address.parse::<std::net::IpAddr>()?;
    let port = config.server.port;

    // Listener
    let mut listener = TcpListener::bind((addr, port)).await?;
    // listener.set_nonblocking(true);

    // TLS
    let der = std::fs::read(config.cert.path)?;
    let cert = native_tls::Identity::from_pkcs12(&der, config.cert.password.as_str())?;
    let tls_acceptor = tokio_tls::TlsAcceptor::from(native_tls::TlsAcceptor::builder(cert).build()?);

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    loop {

        let (stream, remote_addr) = listener.accept().await?;

        // stream.

        // Block blacklisted ips. Drop incoming connections.
        let addr = remote_addr.ip().to_string();
        if config.blacklist.contains(&addr) {
            println!("[blacklist] Dropping connection from {}", addr);
            drop(stream.shutdown(Shutdown::Both));
            drop(stream);
            drop(remote_addr);
            continue;
        }

        let tls_acceptor = tls_acceptor.clone();
        let clients_clone = Arc::clone(&clients);
        tokio::spawn(async move {
            let _ = match tls_acceptor.accept(stream).await {
                Ok(stream) => handle_client(stream, clients_clone).await,
                Err(e) => Err(new_err!(e))
            };
        });

    }

}
