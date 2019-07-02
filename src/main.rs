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

use actix_redis::{RedisActor};
use actix_web::{middleware, web, App, HttpServer};
use std::env;

mod api;
mod model;
use api::feed::{add_item, delete_item_by_id, edit_item, get_all_items, get_item_by_id};
use api::home::index;

fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or("8000".to_string());

    HttpServer::new(move || {
        let redis_url = env::var("REDIS_URL").unwrap_or("localhost:6379".to_string());

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
    .bind(format!("0.0.0.0:{}", port))?
    .run()
}
