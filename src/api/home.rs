use actix_web::{get, web, HttpResponse};
use mime_guess::from_path;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

#[get("/")]
pub async fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Vec<u8> = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes,
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().to_string())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/dist/{_:.*}")]
pub async fn dist(path: web::Path<String>) -> HttpResponse {
    handle_embedded_file(&path.0)
}
