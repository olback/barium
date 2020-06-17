use {
    iron::{Handler, Request, Response, IronResult, status},
    serde::Serialize,
    serde_json,
    crate::CLIENT_COUNT,
    std::sync::atomic::Ordering
};

#[derive(Serialize)]
pub struct Stats {
    version: &'static str,
    users_online: u16
}

impl Stats {

    fn load() -> Self {

        Self {
            version: env!("CARGO_PKG_VERSION"),
            users_online: CLIENT_COUNT.load(Ordering::SeqCst)
        }

    }

    pub fn handle_get(req: &mut Request) -> IronResult<Response> {

        Self::load().handle(req)

    }

}

impl Handler for Stats {

    fn handle(&self, _: &mut Request) -> IronResult<Response> {

        let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();

        Ok(Response::with((
            content_type,
            status::Ok,
            serde_json::to_string_pretty(&self).unwrap()
        )))

    }

}
