use crate::api::{
    feed::{add_item, delete_item_by_id, edit_item, get_all_items, get_item_by_id},
    home::index,
};
use crate::model::AppState;
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
    let redis_url = env::var("REDIS_URL").unwrap_or("localhost:6379".to_string());
    App::with_state(AppState {
        redis_addr: Arc::new(RedisActor::start(redis_url)),
    })
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))
        .configure(|app| {
            Cors::for_app(app)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::CONTENT_TYPE,
                ])
                .resource("/feed/{id}", |r| {
                    r.method(Method::GET).with_async(get_item_by_id);
                    r.method(Method::DELETE).with_async(delete_item_by_id);
                    r.method(Method::PUT).with_async(edit_item);
                })
                .resource("/feed", |r| {
                    r.method(Method::POST).with_async(add_item);
                    r.method(Method::GET).with_async(get_all_items);
                })
                .register()
        })
}
