extern crate gotham;
extern crate futures;
extern crate mime;

use futures::{future, Future, Stream};

use gotham::helpers::http::response::create_response;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};

use hyper::{Body, StatusCode};


fn main() {
    let addr = "0.0.0.0:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router());
}

fn hello_world(mut state: State) -> Box<HandlerFuture>{
    let f = "Hello World";
    println!("{}", f);

     let response = create_response(
                    &state, 
                    StatusCode::OK,
                    mime::TEXT_JAVASCRIPT,
                    f
                    );
    Box::new(future::ok((state, response)))
}

//Build our Gotham router
pub fn router() -> Router {
    build_simple_router(
    |route| {
        route.associate("/secret", 
        |assoc| { 
            assoc.post().to(hello_world);
        });
    })
}