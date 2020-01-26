// https://github.com/jonhoo/rust-evmap

use std::{
    env,
    net::{
        TcpListener,
        TcpStream,
        Shutdown
    },
    io::{
        Read,
        Write
    }
};

mod config;
mod error;
mod macros;

use config::Config;
use error::BariumResult;

fn handle_client(mut stream: TcpStream) -> BariumResult<()> {

    let mut buf = [0 as u8; 8192];

    while match stream.read(&mut buf[..]) {

        Ok(size) if size == 0 => {
            stream.shutdown(Shutdown::Both)?;
            return Ok(())
        },

        Ok(size) => {
            println!("{}: {:?}", size, &buf[0..size]);
            stream.write(&buf[0..size])?;
            true
        },

        Err(e) => {
            eprintln!("{:#?}", e);
            stream.shutdown(Shutdown::Both)?;
            false
        }

    } {}

    Ok(())

}

fn main() -> BariumResult<()> {

    println!("Starting Barium Server...");

    let config = Config::load(env::args().nth(1))?;
    println!("{:#?}", config);

    let listener = TcpListener::bind((config.address.as_str(), config.port)).unwrap();

    for stream in listener.incoming() {

        match stream {
            Ok(s) => {
                std::thread::spawn(move || {
                    handle_client(s)
                });
            },
            Err(e) => eprintln!("{:#?}", e)
        }

    }

    Ok(())

}
