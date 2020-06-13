pub fn serialize_u8_32_arr<S>(u8_32_arr: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {

    let b62 = base62::encode(u8_32_arr);

    serializer.serialize_str(b62.as_str())

}

pub fn deserialize_u8_32_arr<'de, D>(de: D) -> Result<[u8; 32], D::Error>
    where D: serde::Deserializer<'de> {

    use std::convert::TryInto;

    let u8_32_arr_str: &str = serde::de::Deserialize::deserialize(de)?;
    let u8_32_arr = base62::decode(u8_32_arr_str).map_err(serde::de::Error::custom)?;

    u8_32_arr.as_slice().try_into().map_err(serde::de::Error::custom)

}
