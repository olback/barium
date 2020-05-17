use barium_shared::*;
use std::net::TcpStream;
use std::io::{Read, Write};
use rmp_serde;
use rsa;
use rand;
use native_tls;

fn main() -> std::io::Result<()> {

    let mut rng = rand::thread_rng();

    // let user_id = [0u8; 32];
    let private_key = rsa::RSAPrivateKey::new(&mut rng, 4096).unwrap();
    let public_key = private_key.to_public_key();

    // let mut stream = TcpStream::connect(("127.0.0.1", 13337))?;
    // let mut data = rmp_serde::to_vec(&ToServer::Hello(user_id, public_key)).unwrap();

    // stream.write_all(&mut data[..]).unwrap();

    // let mut buf = Vec::<u8>::with_capacity(8);
    // std::io::stdin().read(&mut buf).unwrap();

    // stream.shutdown(std::net::Shutdown::Both).unwrap();

    //////////////////////////////////////

    // for _ in 0..3 {

    //     std::thread::spawn(move || {

    //         let mut stream = TcpStream::connect(("olback.net", 13337)).unwrap();
    //         stream.set_read_timeout(Some(std::time::Duration::from_secs(2))).unwrap();
    //         let mut data = rmp_serde::to_vec(&ToServer::Ping).unwrap();

    //         loop {

    //             // let before = std::time::Instant::now();
    //             stream.write_all(&mut data[..]).unwrap();

    //             // let mut buf = [0u8; 1024];
    //             // stream.read(&mut buf).unwrap();
    //             // println!("Time: {}ms", before.elapsed().as_millis());

    //             // let pong: ToClient = bincode::deserialize(&buf[..]).unwrap();
    //             // println!("pong? {:#?}", pong);

    //         }

    //     });

    // }

    // let mut inbuf = Vec::<u8>::new();
    // std::io::stdin().read(&mut inbuf).unwrap();

    // Ok(())

    let stream = TcpStream::connect("127.0.0.1:13337")?;

    let tls_connector = native_tls::TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
    let mut tls_stream = tls_connector.connect("localhost", stream).unwrap();

    let hello = ToServer::Hello([0u8; 32], public_key, Some("hello".into()));
    let mut hello_bytes = rmp_serde::to_vec(&hello).unwrap();
    tls_stream.write_all(&mut hello_bytes).unwrap();

    let keep_alive = ToServer::KeepAlive([0u8; 32], vec![[158, 98, 145, 151, 12, 180, 77, 217, 64, 8, 199, 155, 202, 249, 216, 111, 24, 180, 180, 155, 165, 178, 160, 71, 129, 219, 113, 153, 237, 59, 158, 78]], AfkStatus::DoNotDisturb);
    let mut keep_alive_bytes = rmp_serde::to_vec(&keep_alive).unwrap();
    tls_stream.write_all(&mut keep_alive_bytes).unwrap();

    let mut pong_bytes = [0u8; 1024];

    match tls_stream.read(&mut pong_bytes) {

        Ok(len) => {
            let pong: ToClient = rmp_serde::from_read_ref(&pong_bytes[0..len]).unwrap();
            println!("{:#?}", pong);
        },
        Err(_) => {}

    }

    Ok(())

}
