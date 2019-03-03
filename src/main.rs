#[macro_use]
extern crate serde_derive;
extern crate actix_redis;
extern crate actix_web;
extern crate chrono;
extern crate futures;
extern crate num_cpus;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate redis_async;

use actix_web::{actix::System, server};
use std::env;

mod api;
mod model;
mod router;
mod share;

fn main() {
    let port = env::var("PORT").expect("missing PORT variable");

    let sys = System::new("api");
    server::new(move || router::app_state())
        .bind(format!("0.0.0.0:{}", &port))
        .expect("Address already in use")
        .shutdown_timeout(5)
        .start();
    println!("app started on port {}", port);
    sys.run();
}