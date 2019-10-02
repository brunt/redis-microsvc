use actix_web::{web, HttpResponse};
use mime_guess::from_path;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

pub fn index() -> HttpResponse {
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

pub fn dist(req: web::HttpRequest) -> HttpResponse {
    let path = &req.path()["/".len()..];
    handle_embedded_file(path)
}
