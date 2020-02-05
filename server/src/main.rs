mod config;
mod error;
mod macros;
mod client;
mod utils;
mod logger;

use config::Config;
use error::BariumResult;
use client::{Client, Clients};
use std::{
    env,
    net::{Shutdown, TcpStream, TcpListener},
    io::Read,
    collections::HashMap,
    sync::{Arc, RwLock}
};
use padlock;
use bincode;
use barium_shared::{AfkStatus, ToClient, ToServer};
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
                break;
            },

            Ok(len) => {

                match bincode::deserialize::<ToServer>(&buf[0..len]) {

                    Ok(data) => {

                        match data {

                            ToServer::KeepAlive(sender, friends, status) => {

                                let hash = utils::sha3_256(sender);

                                let new_user = padlock::rw_read_lock(&clients, |lock| {

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

                                            false
                                        },

                                        None => true

                                    }

                                });

                                if new_user {

                                    client_hash = Some(hash);

                                    let new_client = Client::new(sender, status, &stream)?;

                                    padlock::rw_write_lock(&clients, |lock| {

                                        lock.insert(hash, new_client);
                                        let user = lock.get(&hash).unwrap(); // Safe unwrap since we JUST inserted the value.

                                        let mut friends_online = Vec::<([u8; 32], AfkStatus)>::new();
                                        for friend in &friends {
                                            match lock.get(friend) {
                                                Some(fc) => friends_online.push((friend.clone(), fc.get_idle())),
                                                None => ()
                                            }
                                        }

                                        let _ = user.send_data(ToClient::FriendsOnline(friends_online));


                                    });

                                }

                            },

                            ToServer::Message(message) => {

                                padlock::rw_read_lock(&clients, |lock| {
                                    match lock.get(&message.to) {
                                        Some(user) => user.send_data(ToClient::Message(message.data)),
                                        None => Ok(())
                                    }
                                })?;

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
        let addr = remote_addr.ip().to_string();
        debug!("New connection from {}", addr);

        if config.blacklist.contains(&addr) {
            warn!("Blacklist dropping connection from {}", addr);
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
