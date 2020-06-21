use {
    std::{time::Duration, io::{self, Read, Write},
    sync::mpsc::{self, TryRecvError}, thread},
    glib::{self, MainContext, Priority},
    barium_shared::{ToClient, ToServer},
    rmp_serde,
    crate::utils::new_tls_stream
};

#[derive(Debug)]
pub enum ServerStatus {
    Online,
    Offline
}

pub struct Connection {
    pub cert_rx: glib::Receiver<native_tls::Certificate>,
    pub server_status_rx: glib::Receiver<ServerStatus>,
    pub msg_rx: glib::Receiver<ToClient>,
    pub msg_tx: mpsc::Sender<ToServer>,
}

pub fn connect(address: String, port: u16, allow_invalid_cert: bool) -> Connection {

    let (cert_tx, cert_rx) = MainContext::channel::<native_tls::Certificate>(Priority::default());
    let (server_status_tx, server_status_rx) = MainContext::channel::<ServerStatus>(Priority::default());
    let (msg_tx, external_msg_rx) = MainContext::channel::<ToClient>(Priority::default());
    let (external_msg_tx, msg_rx) = mpsc::channel::<ToServer>();

    thread::spawn(move || loop {

        let mut tls_stream = match new_tls_stream(address.clone(), port, allow_invalid_cert) {
            Ok(ts) => ts,
            Err(_) => {
                std::thread::sleep(Duration::from_secs(1));
                continue
            }
        };

        // TODO: Make sure this works!
        while let Err(e) = tls_stream.get_mut().set_nonblocking(true) {
            eprintln!("{:#?}", e);
            std::thread::sleep(Duration::from_nanos(10));
        }

        // Allocate buffer
        let mut buf = vec![0u8; 8192].into_boxed_slice();

        // Send server status
        drop(server_status_tx.send(ServerStatus::Online));

        // Send cert
        if let Some(cert) = tls_stream.peer_certificate().unwrap_or(None) {
            drop(cert_tx.send(cert));
        };

        loop {

            match msg_rx.try_recv() {

                Ok(ref msg) => {
                    if let Ok(data) = rmp_serde::to_vec(msg) {
                        drop(tls_stream.write_all(&data[..]));
                    }
                },

                Err(e) if e == TryRecvError::Empty => {}, // Do nothing

                Err(e) => {

                    eprintln!("{:#?}", e);
                    break;

                }

            }

            match tls_stream.read(&mut buf) {

                Ok(len) => if let Ok(data) = rmp_serde::from_read_ref::<_, ToClient>(&buf[0..len]) {

                    drop(msg_tx.send(data));

                },

                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}, // Do nothing

                Err(e) => { // broken pipe?

                    eprintln!("{:#?}", e);
                    break;

                }

            }

            thread::sleep(Duration::from_millis(100));

        }

        drop(server_status_tx.send(ServerStatus::Offline));
        drop(tls_stream.shutdown());

    });

    Connection {
        cert_rx: cert_rx,
        server_status_rx: server_status_rx,
        msg_rx: external_msg_rx,
        msg_tx: external_msg_tx,
    }

}
