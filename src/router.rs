//use crate::api::{feed::add_item, feed::get_item_by_key, feed::get_all_items, home::index};
use crate::api::{feed::add_item, feed::get_item_by_key, home::index};
use crate::share::common::AppState;
use actix_redis::RedisActor;
use actix_web::{
    http::{header, Method},
    middleware,
    middleware::cors::Cors,
    App,
};
use std::env;
use std::sync::Arc;

pub fn app_state() -> App<AppState> {
    let redis_url = env::var("REDIS_URL").expect("Missing REDIS_URL");
    App::with_state(AppState {
        redis_addr: Arc::new(RedisActor::start(redis_url)),
    })
    .middleware(middleware::Logger::default())
    .resource("/", |r| r.f(index))
    .configure(|app| {
        Cors::for_app(app)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
//            .resource("/feed/all", |r| r.method(Method::GET).with_async(get_all_items))
            .resource("/feed", |r| {
                r.method(Method::POST).with_async(add_item);
                r.method(Method::GET).with_async(get_item_by_key);
            })
            .register()
    })
}
