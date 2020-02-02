use sha3::{Digest, Sha3_256};

pub fn sha3_256(input: [u8; 32]) -> [u8; 32] {

    let mut hasher = Sha3_256::new();
    hasher.input(input);
    let result = hasher.result();

    let mut output = [0u8; 32];
    for i in 0..result.len() {
        output[i] = result[i];
    }

    println!("{}: {:?}", std::file!(), &output[..]);

    output

}
