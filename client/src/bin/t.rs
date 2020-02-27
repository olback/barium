use barium_shared::*;
use std::net::TcpStream;
use std::io::{Read, Write};
use bincode;
use rsa;
use rand;

fn main() -> std::io::Result<()> {

    let mut rng = rand::thread_rng();

    let user_id = [0u8; 32];
    let private_key = rsa::RSAPrivateKey::new(&mut rng, 512).unwrap();
    let public_key = private_key.to_public_key();

    let mut stream = TcpStream::connect(("127.0.0.1", 13337))?;
    let mut data = bincode::serialize(&ToServer::Hello(user_id, public_key)).unwrap();

    stream.write_all(&mut data[..]).unwrap();

    let mut buf = Vec::<u8>::with_capacity(8);
    std::io::stdin().read(&mut buf).unwrap();

    stream.shutdown(std::net::Shutdown::Both).unwrap();

    Ok(())

}
