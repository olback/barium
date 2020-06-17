use {
    barium_shared::UserId,
    rand::{self, Rng}
};

pub fn new_user_id() -> UserId {

    let mut rng = rand::thread_rng();
    let id = rng.gen::<UserId>();

    id

}
