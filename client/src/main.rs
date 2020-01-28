use std::{
    net::{
        TcpStream,
        Shutdown
    },
    io::{
        Read,
        Write
    }
};
use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
use bincode;

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

    let mut stream = TcpStream::connect(("127.0.0.1", 8080))?;

    let stream_clone = stream.try_clone()?;
    std::thread::spawn(|| {
        let mut stream_clone = stream_clone;
        let mut buf = [0u8; 1024];
        loop {
            match stream_clone.read(&mut buf[..]) {
                Ok(len) if len == 0 => {
                    stream_clone.shutdown(Shutdown::Both)?;
                    break;
                },
                Ok(len) => println!("{:?}", &buf[0..len]),
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(e)
                }
            }
        }
        Ok(())
    });

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.write_all(&mut [65, 65])?;

    std::thread::sleep(std::time::Duration::from_secs(1));

    stream.shutdown(Shutdown::Both)?;

    Ok(())

}
