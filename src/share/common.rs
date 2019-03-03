use actix_redis::RedisActor;
use actix_web::actix::Addr;
use std::sync::Arc;

pub struct AppState {
    pub redis_addr: Arc<Addr<RedisActor>>,
}
