#[macro_use]
extern crate rust_embed;
use actix_redis::RedisActor;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::env;
use std::fs::File;
use std::io::BufReader;

mod api;
mod model;

use crate::api::feed::{add_item, delete_item_by_id, edit_item, get_all_items, get_item_by_id};
use crate::api::home::{dist, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| {
        println!("using default port 8000");
        "8000".to_string()
    });

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file =
        &mut BufReader::new(File::open("openssl/localhost.crt").expect("cert not found"));
    let key_file = &mut BufReader::new(File::open("openssl/localhost.key").expect("key not found"));
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| {
        println!("using localhost redis url");
        "localhost:6379".to_string()
    });

    let prometheus = PrometheusMetrics::new("microsvc", Some("/metrics"), None);

    HttpServer::new(move || {
        let r = RedisActor::start(redis_url.clone());
        App::new()
            .data(r)
            .wrap(middleware::Logger::default())
            .wrap(prometheus.clone())
            .service(
                web::resource("/feed")
                    .route(web::post().to(add_item))
                    .route(web::get().to(get_all_items)),
            )
            .service(
                web::resource("/feed/{id}")
                    .route(web::get().to(get_item_by_id))
                    .route(web::put().to(edit_item))
                    .route(web::delete().to(delete_item_by_id)),
            )
            .service(index)
            .service(dist)
    })
    .bind_rustls(format!("0.0.0.0:{}", port), config)?
    .run()
    .await
}
