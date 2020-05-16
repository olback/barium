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
use std::sync::{Arc, RwLock};
use padlock;
use tray_indicator::TrayIndicator;

mod data;
mod error;
mod macros;
mod keys;
mod resources;
mod ui;
mod notification;
use ui::{
    Ui,
    startup_keygen::StartupKeygenStatus
};

fn main() -> error::BariumResult<()> {

    let config = Arc::new(RwLock::new(data::Config::load()?));
    let keys = Arc::new(RwLock::new(keys::KeyStore::new()));
    println!("{:#?}", config);
    println!("{:#?}", keys);

    // Load resources
    resources::load();

    // Create application
    let application = gtk::Application::new(Some("net.olback.barium"), Default::default())?;

    // Set default icon
    gtk::Window::set_default_icon_name("net.olback.Barium");

    // Create builder
    let builder = gtk::Builder::new_from_resource("/net/olback/barium/ui");

    let config_clone = Arc::clone(&config);
    application.connect_activate(move |app| {

        // Create tray indicator
        let mut tray = TrayIndicator::new("Barium", "net.olback.Barium");
        tray.set_attention_icon("net.olback.BariumAttention");

        let app_clone = app.clone();
        tray.add_menu_item("Show", move |_| {
            let windows = app_clone.get_windows();
            if windows.len() > 0 {
                // Probably not ideal to assume that the first entry is the main window
                windows[0].show();
                windows[0].present();
            }
        });

        let app_clone = app.clone();
        tray.add_menu_item("Quit", move |_| {
            app_clone.quit();
        });

        tray.show(false);

        // Build Ui
        let ui = Ui::build(&app, &builder);

        // Generate keys
        let keys_clone = Arc::clone(&keys);
        let servers_clone = padlock::rw_read_lock(&config_clone, |lock| { lock.servers().clone() });
        let tx = ui.startup_keygen.get_tx();
        std::thread::spawn(move || {
            for s in servers_clone {
                padlock::rw_write_lock(&keys_clone, |lock| {
                    tx.send(StartupKeygenStatus::Generating(s.key_size, format!("{}:{}", s.address, s.port))).unwrap();
                    lock.add(&s).unwrap();
                })
            }
            tx.send(StartupKeygenStatus::Done).unwrap();
        });

    });

    application.run(&args().collect::<Vec<String>>());

    padlock::rw_read_lock(&config, |lock| {
        lock.save()
    })?;

    Ok(())

}
