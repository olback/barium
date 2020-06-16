pub type UserHash = [u8; 32];
pub type UserId = [u8; 32];
pub type KeyBust = u32;

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for [u8; 32] {

    fn to_hex(&self) -> String {

        let mut s = String::with_capacity(64);

        for b in self {
            s.push_str(&format!("{:02x}", b));
        }

        s

    }

}
