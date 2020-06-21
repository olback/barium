use sha3::{Digest, Sha3_256};

pub fn sha3_256(input: &[u8; 32]) -> [u8; 32] {

    let mut hasher = Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();

    let mut output = [0u8; 32];
    let slice = result.iter().map(|&d| d).collect::<Vec<u8>>();
    output.copy_from_slice(&slice[..32]);

    output

}
