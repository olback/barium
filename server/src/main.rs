// https://github.com/jonhoo/rust-evmap

// FIXME: Fix these definitions...

// [0][1..n]
// [0] Message type id
// [1..n] Data

// Keep-alive
// Message type id: 1
// Data [1..33]: user-id (32 bytes)
// Total length: 33 bytes

// Message
// Message type id: 2
// Data [1..33]: destination (32 bytes)
// Data [33..n]: message (n - 33 bytes)

// Get users status
// Message type id: 3
// Data [1..n]: Vec<[u8; 32]>

mod config;
mod error;
mod macros;

use config::Config;
use error::BariumResult;

use std::{
    env,
    net::Shutdown,
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        RwLock,
        mpsc
    }
};
use tokio::{
    net::TcpListener,
    io::{
        AsyncReadExt,
        AsyncWriteExt
    }
};

struct Client {
    idle: RwLock<u32>,
    sender: Mutex<mpsc::Sender<Vec<u8>>>
}

type Clients = Arc<RwLock<HashMap<[u8; 32], Client>>>;

async fn handle_client(mut stream: tokio_tls::TlsStream<tokio::net::TcpStream>, clients: Clients) -> BariumResult<()> {

    let mut buf = [0u8; 8192];
    // let mut tls_stream = tls_acceptor.accept(stream).await?;

    loop {

        match stream.read(&mut buf[..]).await {

            Ok(len) if len == 0 => {
                stream.shutdown();
                return Ok(())
            },

            Ok(len) => {
                // parse data
                stream.write(&[&[0xff], &buf[0..len]].concat()).await?;
                println!("{:?}", &buf[0..len]);
            },

            Err(e) => {
                let err = new_err!(e);
                eprintln!("{}", err);
                return Err(err)
            }

        }

    }

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

    // TLS
    let der = std::fs::read(config.cert.path)?;
    let cert = native_tls::Identity::from_pkcs12(&der, config.cert.password.as_str())?;
    let tls_acceptor = tokio_tls::TlsAcceptor::from(native_tls::TlsAcceptor::builder(cert).build()?);

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    loop {

        let (stream, remote_addr) = listener.accept().await?;

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
