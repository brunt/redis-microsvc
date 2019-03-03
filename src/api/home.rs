use crate::share::common::AppState;
use actix_web::HttpRequest;

pub fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "home"
}
