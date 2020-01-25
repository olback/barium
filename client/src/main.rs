use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
use base64;
use bincode;

mod data;

fn main() {

    let mut rng = rand::thread_rng();

    // rsa key
    let priv_key = RSAPrivateKey::new(&mut rng, 4096).unwrap();
    println!("{:?}", priv_key);

    let priv_key_b64 = base64::encode(&bincode::serialize(&priv_key).unwrap());
    println!("{}", priv_key_b64);

    let pub_key = priv_key.to_public_key();
    println!("{:?}", pub_key);

    let pub_key_b64 = base64::encode(&bincode::serialize(&pub_key).unwrap());
    println!("{:?}", pub_key_b64);
    let pub_key_2: rsa::RSAPublicKey = bincode::deserialize(&base64::decode(&pub_key_b64).unwrap()).unwrap();

    assert_eq!(pub_key, pub_key_2);

    let encrypted = pub_key_2.encrypt(&mut rng, PaddingScheme::PKCS1v15, &[65, 65]).unwrap();
    println!("{:?}", encrypted);

    let decrypted = priv_key.decrypt(PaddingScheme::PKCS1v15, &encrypted).unwrap();
    println!("{:?}", decrypted);

}
