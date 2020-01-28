// https://github.com/jonhoo/rust-evmap

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

mod config;
mod error;
mod macros;

use config::Config;
use error::BariumResult;

use std::{
    env,
    net::{
        Shutdown
    }
};
use tokio::{
    net::{
        TcpListener,
        TcpStream
    },
    io::{
        AsyncReadExt,
        AsyncWriteExt
    }
};

async fn handle_client(mut stream: TcpStream) -> BariumResult<()> {

    let mut buf = [0u8; 8192];

    loop {

        match stream.read(&mut buf[..]).await {

            Ok(len) if len == 0 => {
                stream.shutdown(Shutdown::Both)?;
                return Ok(())
            },

            Ok(len) => {
                // parse data
                stream.write(&buf[0..len]).await?;
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

    let config = Config::load(env::args().nth(1))?;
    println!("{:#?}", config);

    let addr = config.address.parse::<std::net::IpAddr>()?;
    let port = config.port;

    let mut listener = TcpListener::bind((addr, port)).await?;

    loop {

        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = handle_client(stream).await;
        });

    }

}
