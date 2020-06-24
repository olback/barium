use {
    std::{time::Duration, io::{self, Read, Write},
    sync::mpsc::{self, TryRecvError}, thread, net::Shutdown},
    glib::{self, MainContext, Priority},
    barium_shared::{UserId, ToClient, ToServer},
    rmp_serde,
    crate::{KEY_PAIR, utils::new_tls_stream},
    log::{info, warn, error}
};

#[derive(Debug)]
pub enum ServerStatus {
    Online,
    Offline
}

pub struct Connection {
    pub cert_rx: glib::Receiver<Vec<u8>>,
    pub server_status_rx: glib::Receiver<ServerStatus>,
    pub msg_rx: glib::Receiver<ToClient>,
    pub msg_tx: mpsc::Sender<ToServer>,
}

pub fn connect(address: String, port: u16, allow_invalid_cert: bool, id: UserId, password: Option<String>) -> Connection {

    let (cert_tx, cert_rx) = MainContext::channel::<Vec<u8>>(Priority::default());
    let (server_status_tx, server_status_rx) = MainContext::channel::<ServerStatus>(Priority::default());
    let (msg_tx, external_msg_rx) = MainContext::channel::<ToClient>(Priority::default());
    let (external_msg_tx, msg_rx) = mpsc::channel::<ToServer>();

    thread::spawn(move || loop {

        let mut tls_stream = match new_tls_stream(address.clone(), port, allow_invalid_cert) {
            Ok(ts) => ts,
            Err(_) => {
                // If the connection fails, retry every 5 seconds
                error!("Server {}:{} unreachable", address, port);
                thread::sleep(Duration::from_secs(5));
                continue
            }
        };

        info!("TLS Stream established with {}:{}", address, port);

        // Allocate buffer
        let mut buf = vec![0u8; 8192].into_boxed_slice();

        // Authenticate
        let auth = ToServer::Hello(id, KEY_PAIR.public_key().clone(), KEY_PAIR.key_bust(), password.clone());
        let auth_data = rmp_serde::to_vec(&auth).expect("Auth serialization failed");
        drop(tls_stream.write_all(&auth_data[..]));

        match tls_stream.read(&mut buf) {

            Ok(len) => if let Ok(data) = rmp_serde::from_read_ref::<_, ToClient>(&buf[0..len]) {

                match data {
                    ToClient::PasswordOk(ok) => if !ok {
                        warn!("{}:{}: Invalid server password", address, port);
                        return
                    },
                    _ => {
                        error!("Received invalid data from {}:{} during authentication", address, port);
                        return;
                    }
                }

            } else {

                error!("Received invalid data from {}:{} during authentication", address, port);
                continue

            },

            Err(_) => continue

        }

        info!("Password OK for {}:{}", address, port);

        // Make sure the stream is set to non blocking mode
        while let Err(e) = tls_stream.get_mut().set_nonblocking(true) {
            error!("{:#?}", e);
            thread::sleep(Duration::from_nanos(10));
        }

        // Send server status
        drop(server_status_tx.send(ServerStatus::Online));

        // Send cert
        if let Some(cert) = tls_stream.peer_certificate().unwrap_or(None) {
            if let Ok(der) = cert.to_der() {
                drop(cert_tx.send(der));
            }
        };

        loop {

            match msg_rx.try_recv() {

                Ok(ref msg) => {
                    if let Ok(data) = rmp_serde::to_vec(msg) {
                        drop(tls_stream.write_all(&data[..]));
                    }
                },

                Err(e) if e == TryRecvError::Empty => {}, // Do nothing

                Err(_) => {

                    // Return here because when msg_rx is disonnected, there won't ever
                    // be any more messages.
                    info!("Disconnecting from {}:{} due to the message receiver being dropped", address, port);
                    return;

                }

            }

            match tls_stream.read(&mut buf) {

                Ok(len) if len == 0 => {

                    // Broken pipe
                    warn!("Broken pipe, read 0 bytes");
                    break;

                }

                Ok(len) => if let Ok(data) = rmp_serde::from_read_ref::<_, ToClient>(&buf[0..len]) {

                    drop(msg_tx.send(data));

                },

                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}, // Do nothing

                Err(e) => {

                    // Break here instead of return since we want
                    // to re-connect when a failure happens
                    warn!("Broken pipe: {:#?}", e);
                    break; // FIXME:

                }

            }

            thread::sleep(Duration::from_millis(100));

        } // loop

        warn!("Terminating connection to {}:{}...", address, port);

        drop(server_status_tx.send(ServerStatus::Offline));
        drop(tls_stream.shutdown());
        drop(tls_stream.get_ref().shutdown(Shutdown::Both));
        drop(tls_stream);

        warn!("Connection to {}:{} terminated", address, port);

        thread::sleep(Duration::from_secs(1));

    }/* loop */);

    Connection {
        cert_rx: cert_rx,
        server_status_rx: server_status_rx,
        msg_rx: external_msg_rx,
        msg_tx: external_msg_tx,
    }

}
