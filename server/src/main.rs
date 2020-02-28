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
    net::{Shutdown, TcpStream, TcpListener},
    io::{Read, Write},
    collections::HashMap,
    sync::{Arc, RwLock}
};
use padlock;
use bincode;
use barium_shared::{AfkStatus, ToClient, ToServer, hash::sha3_256};
use log::{debug, info, warn};

async fn handle_client(mut stream: TcpStream, clients: Clients) -> BariumResult<()> {

    let mut buf = [0u8; 8192];
    let mut client_hash: Option<[u8; 32]> = None;

    loop {

        match stream.read(&mut buf[..]) {

            Ok(len) if len == 0 => {
                match client_hash {
                    Some(ref ch) => padlock::rw_write_lock(&clients, |lock| {
                        lock.remove(ch);
                    }),
                    None => ()
                }
                let _ = stream.shutdown(Shutdown::Both);
                // debug!("{:#?}", clients);
                break;
            },

            Ok(len) => {

                match bincode::deserialize::<ToServer>(&buf[0..len]) {

                    Ok(data) => {

                        match data {

                            ToServer::Ping => {

                                let pong = ToClient::Pong;
                                let data = bincode::serialize(&pong)?;

                                debug!("{:?}", data);

                                stream.write_all(&data[..])?;

                            },

                            ToServer::Hello(sender, user_public_key) => {

                                let hash = sha3_256(&sender);

                                let exists = padlock::rw_read_lock(&clients, |lock| {
                                    lock.get(&hash).is_some()
                                });

                                if exists {

                                    match stream.peer_addr() {
                                        Ok(addr) => {
                                            warn!("User from {} tried to recreate a session!", addr);
                                        },
                                        Err(_) => ()
                                    };

                                    let _ = stream.shutdown(Shutdown::Both);
                                    // debug!("{:#?}", clients);
                                    break;

                                } else {

                                    client_hash = Some(hash);

                                    // let new_client = Client::new(sender, AfkStatus::Away(None), &stream)?;
                                    let new_client = Client::new(&stream, user_public_key, AfkStatus::Away(None))?;

                                    padlock::rw_write_lock(&clients, |lock| {

                                        lock.insert(hash, new_client);

                                    });

                                    debug!("{:#?}", clients);

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

                                        None => {}

                                    }

                                });

                            },

                            ToServer::Message(sender, message) => {

                                let hash = sha3_256(&sender);

                                let exists = padlock::rw_read_lock(&clients, |lock| {
                                    lock.get(&hash).is_some()
                                });

                                if exists {

                                    padlock::rw_read_lock(&clients, |lock| {
                                        match lock.get(&message.to) {
                                            Some(user) => user.send_data(ToClient::Message(message.data)),
                                            None => Ok(())
                                        }
                                    })?;

                                }

                            }

                        }

                    },

                    Err(e) => {
                        // Recv invalid data
                        warn!("{}", e);
                        stream.shutdown(Shutdown::Both)?;
                        break;
                    }

                }

            },

            Err(e) => {
                warn!("{}", e);
                match client_hash {
                    Some(ref ch) => padlock::rw_write_lock(&clients, |lock| {
                        lock.remove(ch);
                    }),
                    None => ()
                }
                stream.shutdown(Shutdown::Both)?;
                break;
            }

        } // end of match read

        // debug!("{:#?}", clients);

    } // end of loop

    debug!("Connection closed");

    Ok(())

}

#[tokio::main]
async fn main() -> BariumResult<()> {

    logger::configure()?;

    info!("Starting Barium Server...");

    // Load config
    let config = Config::load(env::args().nth(1)).unwrap_or_default();
    config.log();

    // Listener variables
    let addr = config.server.address.parse::<std::net::IpAddr>()?;
    let port = config.server.port;

    // Listener
    let listener = TcpListener::bind((addr, port))?;

    // Keep track of clients
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    loop {

        let (stream, remote_addr) = listener.accept()?;

        // Block blacklisted ips. Drop incoming connections.
        debug!("New connection from {}", remote_addr);

        let addr = remote_addr.ip().to_string();
        if config.blacklist.contains(&addr) {
            warn!("Blacklist dropping connection from {}", remote_addr);
            drop(stream.shutdown(Shutdown::Both));
            drop(stream);
            drop(remote_addr);
            continue;
        }

        let clients_clone = Arc::clone(&clients);
        tokio::spawn(async move {
            let _ = handle_client(stream, clients_clone).await;
        });

    }

}
