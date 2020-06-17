use iron::{AfterMiddleware, IronResult, Response, Request};

pub struct ServerHeader;

impl AfterMiddleware for ServerHeader {

    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {

        res.headers.append_raw("Server", b"Barium API Server (Iron)".to_vec());

        Ok(res)

    }

}
