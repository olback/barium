use {
    padlock,
    std::sync::{Arc, Mutex},
    crate::{
        servers::{Servers, Server},
        error::BariumResult
    }
};

pub fn add_server(servers: &Arc<Mutex<Servers>>, server: Server) -> BariumResult<()> {

    padlock::mutex_lock(&servers, move |servers| servers.add(server))?;

    Ok(())

}
