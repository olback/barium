use {
    std::{net::{TcpStream}, io::{Read, Write}},
    native_tls::TlsConnector,
    barium_shared::{UserId, KeyBust, ToClient, ToServer, EncryptedMessage, hash::sha3_256},
    rand::{self, Rng},
    base62,
    rsa::{PublicKey, RSAPublicKey, RSAPrivateKey, PaddingScheme},
    rmp_serde
};

fn sign_and_encrypt(pubkey: &RSAPublicKey, privkey: &RSAPrivateKey, data: &[u8]) -> (Vec<u8>, Vec<u8>) {

    let mut rng = rand::thread_rng();

    let sig = privkey.sign(PaddingScheme::PKCS1v15Sign { hash: None }, &data).unwrap();
    let encrypted = pubkey.encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, &data).unwrap();

    (sig, encrypted)

}

fn decrypt_and_verify(pubkey: &RSAPublicKey, privkey: &RSAPrivateKey, data: &[u8], sig:&[u8]) -> Vec<u8> {

    let a = privkey.decrypt(PaddingScheme::PKCS1v15Encrypt, &data).unwrap();
    pubkey.verify(PaddingScheme::PKCS1v15Sign { hash: None }, &a, &sig).unwrap();

    a

}

fn main() {

    let mut rng = rand::thread_rng();
    let id = rng.gen::<UserId>();
    let key_bust = rng.gen::<KeyBust>();
    let private_key = RSAPrivateKey::new(&mut rng, 4096).unwrap();
    let public_key = private_key.to_public_key();

    println!("Id (base62): {}", base62::encode(&id));
    println!("Hash (base62): {}", base62::encode(&sha3_256(&id)));

    let stream = TcpStream::connect("133.barium.chat:13337").unwrap();
    let tls_connector = TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
    let mut tls_stream = tls_connector.connect("133.barium.chat", stream).unwrap();

    let hello = ToServer::Hello(id, public_key, key_bust, Some("hello".into()));
    let hello_data = rmp_serde::to_vec(&hello).unwrap();

    tls_stream.write_all(&hello_data).unwrap(); // Connect
    let mut buf = [0u8; 2048];

    let mut friend_hash_str = String::new();
    std::io::stdin().read_line(&mut friend_hash_str).unwrap();
    let friend_hash: [u8; 32] = slice_to_arr(&base62::decode(friend_hash_str.trim()).unwrap());

    let public_key_request = ToServer::GetPublicKeys(id, vec![friend_hash]);
    let public_key_request_data = rmp_serde::to_vec(&public_key_request).unwrap();
    tls_stream.write_all(&public_key_request_data).unwrap();

    let len = tls_stream.read(&mut buf).unwrap();
    let friend_public_key = match rmp_serde::from_read_ref::<_, ToClient>(&buf[0..len]).unwrap() {
        ToClient::PublicKeys(keys) => keys[0].2.clone(),
        _ => panic!("Did not get public key")
    };

    let unique_data = rng.gen::<[u8; 4]>();

    std::thread::sleep(std::time::Duration::from_secs(2));

    loop {

        let (sig, ciphertext) = sign_and_encrypt(&friend_public_key, &private_key, &unique_data);
        let body = EncryptedMessage::new(ciphertext, sig);

        let payload = rmp_serde::to_vec(&ToServer::Message(id, friend_hash, body)).unwrap();
        tls_stream.write_all(&payload).unwrap();

        let len = tls_stream.read(&mut buf).unwrap();
        let incomming: ToClient = match rmp_serde::from_read_ref(&buf[0..len]) {
            Ok(m) => m,
            Err(_) => continue
        };

        match incomming {

            ToClient::Message(m) => {

                let res = decrypt_and_verify(&friend_public_key, &private_key, m.ciphertext(), m.signature());
                println!("{}: {:?}", res.len(), res);

            },

            ToClient::Error(_, err) => eprintln!("{}", err),

            _ => {}

        }

        std::thread::sleep(std::time::Duration::from_secs(1));

    }

}

fn slice_to_arr(slice: &[u8]) -> [u8; 32] {

    assert!(slice.len() == 32);

    let mut arr = [0u8; 32];
    arr.copy_from_slice(slice);

    arr

}
