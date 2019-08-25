extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate futures;
extern crate mime;
extern crate serde;
extern crate serde_json;

use futures::future;

use gotham::helpers::http::response::create_response;
use gotham::handler::HandlerFuture;
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single_middleware;
use gotham::pipeline::single::single_pipeline;

use hyper::StatusCode;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, ListSecretsRequest, SecretsManager, SecretsManagerClient};

#[derive(Debug, Serialize, Deserialize)]
struct Secret{
    #[serde(rename="Test")]
    test: String,
}

#[derive(Clone, StateData)]
struct Substr8 {
    secrets: HashMap<String, String>,
   // pub client: SecretsManagerClient,
}

impl Substr8 {
    fn new() -> Self {


        // Build client for later
        let client = SecretsManagerClient::new(Region::UsWest2);

        // Build Internal Mapping
        let mut secrets = HashMap::new();
        let secret_list = list_secrets(&client);
        for secret in secret_list.iter() {
            for s in secret {
                println!("{}", s.name
                                .clone()
                                .unwrap());
                let val = get_secret(&client, s.name
                                                .clone()
                                                .unwrap());
                secrets.insert(s.name
                                .clone()
                                .unwrap(), val); // Handle errors like an adult
            }
        }

        Self {secrets}
    }
}

// Retrieves an individual secret given a name
fn get_secret(client: &SecretsManagerClient, name: String) -> String {
    let mut req = GetSecretValueRequest::default();
    req.secret_id = name;
    let res = client.get_secret_value(req)
                        .sync();

    let secret = res.unwrap().secret_string.unwrap();
    println!("{:?}", secret);

    let pair: Secret = serde_json::from_str(&secret)
                            .expect("Couldn't deserialize");
    pair.test
}

// Retrieves all secrets in the Secret Manager
fn list_secrets(client: &SecretsManagerClient) -> Option<std::vec::Vec<rusoto_secretsmanager::SecretListEntry>> {
    let req = ListSecretsRequest{
        ..Default::default()
    };
    let result = client.list_secrets(req).sync();
    result.unwrap().secret_list
}

fn main() {

    let middleware = StateMiddleware::new(Substr8::new());
    let pipeline = single_middleware(middleware);

    let (chain, pipelines) = single_pipeline(pipeline);
    let addr = "0.0.0.0:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, build_router(chain, pipelines, |route| {
        route.get("/secrets").to(secrets_handler);
    }));
}


fn secrets_handler(state: State) -> Box<HandlerFuture>{
    let sub = Substr8::borrow_from(&state);
    let secret = sub.secrets["SecretTest"].clone();
    let response = create_response(
                    &state, 
                    StatusCode::OK,
                    mime::TEXT_JAVASCRIPT,
                    secret
                    );
    Box::new(future::ok((state, response)))
}