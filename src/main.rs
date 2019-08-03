#[macro_use]
extern crate serde_derive;
extern crate actix_redis;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate redis_async;
extern crate guid_create;
extern crate rustls;

use std::io::BufReader;
use std::fs::File;
use actix_redis::{RedisActor};
use actix_web::{middleware, web, App, HttpServer};
use std::env;

use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};

mod api;
mod model;
use api::feed::{add_item, delete_item_by_id, edit_item, get_all_items, get_item_by_id};
use api::home::index;

fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| {
        println!("using default port 8000");
        "8000".to_string()
    });

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("openssl/localhost.crt").expect("cert not found"));
    let key_file = &mut BufReader::new(File::open("openssl/localhost.key").expect("key not found"));
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(move || {
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| {
            println!("using localhost redis url");
            "localhost:6379".to_string()
        });

        let r = RedisActor::start(redis_url);
        App::new()
            .data(r)
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/feed")
                    .route(web::post().to_async(add_item))
                    .route(web::get().to_async(get_all_items)),
            )
            .service(
                web::resource("/feed/{id}")
                    .route(web::get().to_async(get_item_by_id))
                    .route(web::put().to_async(edit_item))
                    .route(web::delete().to_async(delete_item_by_id)),
            )
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind_rustls(format!("0.0.0.0:{}", port), config)?
    .run()
}
