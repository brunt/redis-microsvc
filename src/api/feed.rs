use crate::model::{AppState, FeedItem, FeedItemRequest, FeedItemResponse, FeedItemsResponse};
use actix_redis::{Command, Error as ARError};
use actix_web::{AsyncResponder, Error as AWError, HttpRequest, HttpResponse, Json};
use chrono::Local;
use futures::Future;
use guid_create::GUID;
use itertools::Itertools;
use redis_async::resp::RespValue;

pub fn get_item_by_id(
    req: HttpRequest<AppState>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let redis = req.state().redis_addr.clone();
    let id = req
        .match_info()
        .get("id")
        .unwrap_or(&"".to_string())
        .to_owned();
    redis
        .send(Command(resp_array!["HGET", "feeditems", &id]))
        .map_err(AWError::from)
        .and_then(move |res: Result<RespValue, ARError>| match res {
            Ok(RespValue::BulkString(x)) => {
                if let Ok(s) = String::from_utf8(x) {
                    let elements: Vec<String> = s.split("🤔").map(|s| s.to_string()).collect();
                    if elements.len() == 3 {
                        Ok(HttpResponse::Ok().content_type("application/json").json(
                            FeedItemResponse {
                                id: id,
                                title: elements[0].to_owned(),
                                body: elements[1].to_owned(),
                                time: elements[2].to_owned(),
                            },
                        ))
                    } else {
                        Ok(HttpResponse::InternalServerError()
                            .content_type("text/plain")
                            .body("something weird happened"))
                    }
                } else {
                    Ok(HttpResponse::Ok()
                        .content_type("text/plain")
                        .body("not found"))
                }
            }
            Ok(RespValue::Nil) => Ok(HttpResponse::NotFound()
                .content_type("text/plain")
                .body("No record found with that id")),
            _ => {
                println!("--->{:?}", res);
                Ok(HttpResponse::InternalServerError().finish())
            }
        })
        .responder()
}

pub fn delete_item_by_id(
    req: HttpRequest<AppState>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let redis = req.state().redis_addr.clone();
    let id = req
        .match_info()
        .get("id")
        .unwrap_or(&"".to_string())
        .to_owned();
    redis
        .send(Command(resp_array!["HDEL", "feeditems", &id]))
        .map_err(AWError::from)
        .and_then(move |res: Result<RespValue, ARError>| match res {
            Ok(RespValue::Integer(x)) => Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(x.to_string())),
            Ok(RespValue::Nil) => Ok(HttpResponse::NotFound()
                .content_type("text/plain")
                .body("No record found with that id")),
            _ => {
                println!("--->{:?}", res);
                Ok(HttpResponse::InternalServerError().finish())
            }
        })
        .responder()
}

pub fn add_item(
    (feed_item, req): (Json<FeedItemRequest>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let redis = req.state().redis_addr.clone();
    let feed_item_req = feed_item.into_inner();
    let item = FeedItem {
        title: feed_item_req.title.clone(),
        body: feed_item_req.body.clone(),
        time: Local::now().to_string(),
    };
    let id = GUID::rand().to_string().to_lowercase();
    redis
        .send(Command(resp_array![
            "HSET",
            "feeditems",
            &id,
            &item.to_string()
        ]))
        .map_err(AWError::from)
        .and_then(move |res: Result<RespValue, ARError>| match res {
            Ok(_) => {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(FeedItemResponse {
                        id: id,
                        title: item.title,
                        body: item.body,
                        time: item.time,
                    }))
            }
            _ => {
                println!("--->{:?}", res);
                Ok(HttpResponse::InternalServerError().finish())
            },
        })
        .responder()
}

//
pub fn edit_item(
    (feed_item, req): (Json<FeedItemRequest>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let redis = req.state().redis_addr.clone();
    let feed_item_req = feed_item.into_inner();
    let item = FeedItem {
        title: feed_item_req.title.clone(),
        body: feed_item_req.body.clone(),
        time: Local::now().to_string(),
    };
    //the framework returns 404 when path variable is missing so this is fine
    let id = req
        .match_info()
        .get("id")
        .unwrap_or("")
        .to_string()
        .to_lowercase();
    redis
        .send(Command(resp_array![
            "HSET",
            "feeditems",
            &id,
            &item.to_string()
        ]))
        .map_err(AWError::from)
        .and_then(move |res: Result<RespValue, ARError>| match res {
            Ok(_) => {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(FeedItemResponse {
                        id: id.to_string(),
                        title: item.title,
                        body: item.body,
                        time: item.time,
                    }))
            }
            _ => {
                println!("--->{:?}", res);
                Ok(HttpResponse::InternalServerError().finish())
            },
        })
        .responder()
}

pub fn get_all_items(
    req: HttpRequest<AppState>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let redis = req.state().redis_addr.clone();
    redis
        .send(Command(resp_array!["HGETALL", "feeditems"]))
        .map_err(AWError::from)
        .and_then(move |res: Result<RespValue, ARError>| match res {
            Ok(RespValue::Array(x)) => {
                let mut out = FeedItemsResponse { items: Vec::new() };
                for mut item in &x.into_iter().chunks(2) {
                    let k = extract_id(item.next());
                    let (a, b, c) = extract_values(item.next());
                    let f = FeedItemResponse {
                        id: k,
                        title: a,
                        body: b,
                        time: c,
                    };
                    out.items.push(f);
                }
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(out))
            }
            Err(_e) => Ok(HttpResponse::InternalServerError().finish()),
            _ => {
                println!("--->{:?}", res);
                Ok(HttpResponse::InternalServerError().finish())
            }
        })
        .responder()
}

//TODO: proper error handling and bounds checking
fn extract_id(input: Option<RespValue>) -> String {
    if let Some(RespValue::BulkString(value)) = input {
        if let Ok(s) = String::from_utf8(value) {
            return s;
        }
    }
    "".to_string()
}

fn extract_values(input: Option<RespValue>) -> (String, String, String) {
    if let Some(RespValue::BulkString(value)) = input {
        if let Ok(s) = String::from_utf8(value) {
            let elements: Vec<String> = s.split("🤔").map(|s| s.to_string()).collect();
            if elements.len() == 3 {
                return (
                    elements[0].to_owned(),
                    elements[1].to_owned(),
                    elements[2].to_owned(),
                );
            }
        }
    }
    ("".to_string(), "".to_string(), "".to_string())
}
