use std::{
    net::{
        TcpStream,
        Shutdown
    },
    io::{
        Read,
        Write
    },
    sync::Arc
};
use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
use bincode;
use native_tls;
use barium_shared::{AfkStatus, ToClient, ToServer};
use rand::Rng;

mod data;
mod error;
mod macros;
mod message;

use error::BariumResult;
use message::Message;

fn main() -> BariumResult<()> {

    // let message = Message::text("Hello gais! d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d d");
    // let key_size = 4096;
    // let max_data_len = u32::pow(2, (f32::log2(key_size as f32) - 3f32) as u32);

    // println!("Key size: {}, Max data len: {}", key_size, max_data_len - 11);

    // let mut rng = rand::thread_rng();

    // // rsa key
    // let priv_key = RSAPrivateKey::new(&mut rng, key_size)?;
    // let pub_key = priv_key.to_public_key();

    // let message_bytes = bincode::serialize(&message)?;
    // println!("Pub key len: {}, Bytes len: {}", pub_key.size(), message_bytes.len());
    // let encrypted = pub_key.encrypt(&mut rng, PaddingScheme::PKCS1v15, &message_bytes)?;
    // println!("[{}] {:?}", encrypted.len(), encrypted);

    // let mut stream = TcpStream::connect(("0.0.0.0", 8080))?;
    // stream.write_all(&encrypted[..])?;

    // let mut buf = [0u8; 4096];
    // let len = stream.read(&mut buf[..])?;

    // let decrypted = priv_key.decrypt(PaddingScheme::PKCS1v15, &buf[0..len])?;
    // let recv_message: Message = bincode::deserialize(&decrypted[..])?;
    // println!("{:?}", recv_message);

    // stream.shutdown(Shutdown::Both)?;

    // Ok(())

    // let tls_connector = native_tls::TlsConnector::new().unwrap();
    let mut tls_connector = native_tls::TlsConnector::builder();
    tls_connector.danger_accept_invalid_certs(true);
    tls_connector.danger_accept_invalid_hostnames(true);
    tls_connector.use_sni(false);
    tls_connector.min_protocol_version(Some(native_tls::Protocol::Tlsv12));
    let tls_connector = tls_connector.build().unwrap();

    let tcp_stream = TcpStream::connect(("localhost", 13337)).unwrap();
    // tcp_stream.set_nonblocking(true).unwrap();
    tcp_stream.set_read_timeout(Some(std::time::Duration::from_millis(10))).unwrap();
    let mut stream = tls_connector.connect("localhost", tcp_stream).unwrap();

    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();

    std::thread::spawn(move || {
        connection(stream, rx)
    });

    // let mut rng = rand::thread_rng();
    // let id = rng.gen::<[u8; 32]>();
    let id = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let friends = vec![[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]];

    loop {

        let to_server = ToServer::KeepAlive(id, friends.clone(), AfkStatus::Available);
        let data = bincode::serialize(&to_server).unwrap();

        tx.send(data).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

    }

    Ok(())

}

fn connection(mut stream: native_tls::TlsStream<std::net::TcpStream>, rx: std::sync::mpsc::Receiver<Vec<u8>>) -> BariumResult<()> {

    let mut buf = [0u8; 2048];

    loop {

        match rx.try_recv() {
            Ok(msg) => stream.write_all(&msg[..])?,
            Err(e) if e == std::sync::mpsc::TryRecvError::Empty => (),
            Err(e) => return Err(new_err!(e))
        }

        match stream.read(&mut buf[..]) {
            Ok(len) if len == 0 => {
                stream.shutdown()?;
                break;
            },
            Ok(len) => {
                // println!("{:?}", &buf[0..len])
                match bincode::deserialize::<ToClient>(&buf[0..len]) {
                    Ok(data) => println!("{:#?}", data),
                    Err(e) => eprintln!("{:#?}", e)
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(ref e) => {
                eprintln!("{}", e);
                return Err(new_err!(e));
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100))

    }

    Ok(())

}
