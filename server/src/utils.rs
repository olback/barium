use {
    barium_shared::ServerProperties,
    crate::config::Config
};

pub fn get_server_properties(conf: &Config) -> ServerProperties {

    barium_shared::ServerProperties {
        requires_password: conf.server.password.is_some()
    }

}
