use {
    iron::{AfterMiddleware, IronResult, IronError, Response, Request},
    router::NoRoute
};

pub struct Custom404;

impl AfterMiddleware for Custom404 {

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {

        if err.error.is::<NoRoute>() {

            Ok(Response::with((
                iron::status::NotFound,
                "404 Not Found"
            )))

        } else {

            Err(err)

        }

    }

}
