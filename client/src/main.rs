// use std::{
//     net::{
//         TcpStream,
//         Shutdown
//     },
//     io::{
//         Read,
//         Write
//     },
//     sync::Arc
// };
// use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
// use bincode;
// use barium_shared::{AfkStatus, ToClient, ToServer};
// use rand::Rng;

// mod data;
// mod error;
// mod macros;
// mod message;

// use error::BariumResult;
// use message::Message;

// fn main() -> BariumResult<()> {

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

// }

use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;

mod error;
mod macros;
mod resources;
mod ui;
use ui::Ui;

fn main() -> error::BariumResult<()> {

    // Load resources
    resources::load();

    // Create application
    let application = gtk::Application::new(Some("net.olback.barium"), Default::default())?;
    application.set_flags(gio::ApplicationFlags::HANDLES_OPEN);

    // Create builder
    let builder = gtk::Builder::new_from_resource("/net/olback/barium/ui");

    application.connect_activate(move |app| {
        let ui = Ui::build(&app, &builder);
    });

    application.connect_open(|app, files, file| {
        println!("{:#?} {:#?}", files[0], file);
    });

    application.run(&args().collect::<Vec<String>>());

    Ok(())

}
