extern crate gotham;
extern crate futures;
extern crate mime;

#[macro_use]
extern crate lazy_static;

use futures::future;

use gotham::helpers::http::response::create_response;
use gotham::handler::HandlerFuture;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::State;

use hyper::StatusCode;

use std::collections::HashMap;

lazy_static! {
    static ref SECRETS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("test", "hello");
        map
    };
}

fn main() {
    let addr = "0.0.0.0:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router());
}

fn hello_world(state: State) -> Box<HandlerFuture>{
    let f = "Hello World";
    println!("{}", SECRETS["test"]);
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