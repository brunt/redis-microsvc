use crate::model::AppState;
use actix_web::HttpRequest;

pub fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "Routes: GET: /, GET/POST: /feed, GET/PUT/DELETE: /feed/{id}"
}
