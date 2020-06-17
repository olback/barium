mod custom_404;
mod stats;
mod server_header;

use {
    iron::{Iron, Chain},
    router::Router,
    stats::Stats,
    custom_404::Custom404,
    server_header::ServerHeader,
    crate::config::HttpApi,
    log::info
};

pub fn serve(settings: &HttpApi) {

    info!("Starting HTTP API");

    let mut router = Router::new();
    router.get("/stats", Stats::handle_get, "stats");

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);
    chain.link_after(ServerHeader);

    Iron::new(chain).http((settings.address.as_str(), settings.port)).unwrap();

}
