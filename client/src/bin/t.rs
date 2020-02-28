use barium_shared::*;
use std::net::TcpStream;
use std::io::{Read, Write};
use bincode;
use rsa;
use rand;

fn main() -> std::io::Result<()> {

    // let mut rng = rand::thread_rng();

    // let user_id = [0u8; 32];
    // let private_key = rsa::RSAPrivateKey::new(&mut rng, 512).unwrap();
    // let public_key = private_key.to_public_key();

    // let mut stream = TcpStream::connect(("127.0.0.1", 13337))?;
    // let mut data = bincode::serialize(&ToServer::Hello(user_id, public_key)).unwrap();

    // stream.write_all(&mut data[..]).unwrap();

    // let mut buf = Vec::<u8>::with_capacity(8);
    // std::io::stdin().read(&mut buf).unwrap();

    // stream.shutdown(std::net::Shutdown::Both).unwrap();

    //////////////////////////////////////

    for _ in 0..3 {

        std::thread::spawn(move || {

            let mut stream = TcpStream::connect(("olback.net", 13337)).unwrap();
            stream.set_read_timeout(Some(std::time::Duration::from_secs(2))).unwrap();
            let mut data = bincode::serialize(&ToServer::Ping).unwrap();

            loop {

                // let before = std::time::Instant::now();
                stream.write_all(&mut data[..]).unwrap();

                // let mut buf = [0u8; 1024];
                // stream.read(&mut buf).unwrap();
                // println!("Time: {}ms", before.elapsed().as_millis());

                // let pong: ToClient = bincode::deserialize(&buf[..]).unwrap();
                // println!("pong? {:#?}", pong);

            }

        });

    }

    let mut inbuf = Vec::<u8>::new();
    std::io::stdin().read(&mut inbuf).unwrap();

    Ok(())

}
