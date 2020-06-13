use {
    std::fs,
    serde::{Serialize, Deserialize},
    serde_json,
    rand::{self, Rng},
    barium_shared::UserId,
    crate::{
        error::BariumResult,
        utils::{conf_dir, serialize_u8_32_arr, deserialize_u8_32_arr}
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    #[serde(serialize_with="serialize_u8_32_arr", deserialize_with="deserialize_u8_32_arr")]
    id: UserId
}

impl Identity {

    pub fn load() -> BariumResult<Self> {

        let path = conf_dir()?.join("identity.json");

        if path.is_file() {

            Ok(serde_json::from_str(&fs::read_to_string(&path)?)?)

        } else {

            let mut rng = rand::thread_rng();
            let id = rng.gen::<UserId>();

            let identity = Self { id };
            let json = serde_json::to_string_pretty(&identity)?;
            fs::write(&path, &json)?;

            Ok(identity)

        }

    }

}
