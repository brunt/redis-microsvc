use crate::model::{FeedItem, FeedItemRequest, FeedItemResponse, FeedItemsResponse};
use actix::Addr;
use actix_redis::{Command, RedisActor};
use actix_web::{web, HttpResponse};
use chrono::Local;
use guid_create::GUID;
use itertools::Itertools;
use log::{error, info};
use redis_async::resp::RespValue;
use redis_async::resp_array;

pub async fn get_item_by_id(
    redis: web::Data<Addr<RedisActor>>,
    req: web::Path<String>,
) -> HttpResponse {
    let id = req.to_owned();
    let res = redis
        .send(Command(resp_array!["HGET", "feeditems", &id]))
        .await;
    match res {
        Err(e) => {
            println!("actix error happened: {}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(rr) => match rr {
            Err(e) => {
                println!("error happened: {}", e);
                HttpResponse::InternalServerError().finish()
            }
            Ok(RespValue::Nil) => HttpResponse::NotFound()
                .content_type("text/plain")
                .body("no record found with that ID"),
            Ok(RespValue::BulkString(x)) => match String::from_utf8(x) {
                Err(e) => {
                    println!("error happened: {}", e);
                    HttpResponse::NotFound()
                        .content_type("text/plain")
                        .body("not found")
                }
                Ok(s) => {
                    let elements: Vec<String> = s.split("🤔").map(|s| s.to_string()).collect();
                    if elements.len() == 3 {
                        HttpResponse::Ok()
                            .content_type("application/json")
                            .json(FeedItemResponse {
                                id,
                                title: elements[0].to_owned(),
                                body: elements[1].to_owned(),
                                time: elements[2].to_owned(),
                            })
                    } else {
                        println!(
                            "incorrect number of elements returned in result: {}",
                            elements.len()
                        );
                        HttpResponse::InternalServerError().finish()
                    }
                }
            },
            _ => {
                println!("{:?}", rr);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

pub async fn delete_item_by_id(
    redis: web::Data<Addr<RedisActor>>,
    req: web::Path<String>,
) -> HttpResponse {
    let id = req.to_string();
    println!("deleting id {}", &id);
    let res = redis
        .send(Command(resp_array!["HDEL", "feeditems", &id]))
        .await;
    match res {
        Err(e) => {
            println!("actix error happened: {}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(rr) => match rr {
            Ok(RespValue::Nil) => HttpResponse::NotFound()
                .content_type("text/plain")
                .body("No record found with that ID"),
            Ok(RespValue::Integer(x)) => HttpResponse::Ok().body(x.to_string()),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

pub async fn add_item(
    feed_item: web::Json<FeedItemRequest>,
    redis: web::Data<Addr<RedisActor>>,
) -> HttpResponse {
    let feed_item_req = feed_item.into_inner();
    let item = FeedItem {
        title: feed_item_req.title.clone(),
        body: feed_item_req.body.clone(),
        time: Local::now().to_string(),
    };
    let id = GUID::rand().to_string().to_lowercase();
    let res = redis
        .send(Command(resp_array!(
            "HSET",
            "feeditems",
            &id,
            &item.to_string()
        )))
        .await;
    match res {
        Err(e) => {
            println!("actix error happened: {}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(rr) => match rr {
            Ok(_) => HttpResponse::Ok()
                .content_type("application/json")
                .json(FeedItemResponse {
                    id,
                    title: item.title,
                    body: item.body,
                    time: item.time,
                }),
            _ => {
                println!("{:?}", rr);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

pub async fn edit_item(
    feed_item: web::Json<FeedItemRequest>,
    redis: web::Data<Addr<RedisActor>>,
    req: web::Path<String>,
) -> HttpResponse {
    let feed_item_req = feed_item.into_inner();
    let item = FeedItem {
        title: feed_item_req.title.clone(),
        body: feed_item_req.body.clone(),
        time: Local::now().to_string(),
    };
    //the framework returns 404 when path variable is missing so this is fine
    let id = req.to_string().to_lowercase();
    let res = redis
        .send(Command(resp_array!(
            "HSET",
            "feeditems",
            &id,
            &item.to_string()
        )))
        .await;

    match res {
        Err(e) => {
            println!("actix error happened: {}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(rr) => match rr {
            Ok(_) => HttpResponse::Ok()
                .content_type("application/json")
                .json(FeedItemResponse {
                    id,
                    title: item.title,
                    body: item.body,
                    time: item.time,
                }),
            _ => {
                println!("{:?}", rr);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

pub async fn get_all_items(redis: web::Data<Addr<RedisActor>>) -> HttpResponse {
    let res = redis
        .send(Command(resp_array!["HGETALL", "feeditems"]))
        .await;
    match res {
        Err(e) => {
            println!("actix error happened: {}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(rr) => match rr {
            Ok(RespValue::Array(x)) => {
                let mut out = FeedItemsResponse { items: Vec::new() };
                for mut item in &x.into_iter().chunks(2) {
                    let id = extract_id(item.next());
                    let (title, body, time) = extract_values(item.next());
                    let f = FeedItemResponse {
                        id,
                        title,
                        body,
                        time,
                    };
                    out.items.push(f);
                }
                HttpResponse::Ok()
                    .content_type("application/json")
                    .json(out)
            }
            Ok(RespValue::Nil) => {
                HttpResponse::Ok().json(FeedItemsResponse{
                    items: Vec::new()
                })
            },
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_values() {
        let input = Some(RespValue::BulkString("one🤔two🤔three".as_bytes().to_vec()));
        let output = extract_values(input);
        assert_eq!(output.0, "one".to_string());
        assert_eq!(output.1, "two".to_string());
        assert_eq!(output.2, "three".to_string());

        let input = Some(RespValue::BulkString("something".as_bytes().to_vec()));
        let output = extract_values(input);
        assert_eq!(output.0, "".to_string());
        assert_eq!(output.1, "".to_string());
        assert_eq!(output.2, "".to_string());

        let input = Some(RespValue::Array(vec![RespValue::BulkString(
            "something".as_bytes().to_vec(),
        )]));
        let output = extract_values(input);
        assert_eq!(output.0, "".to_string());
        assert_eq!(output.1, "".to_string());
        assert_eq!(output.2, "".to_string());

        let input = Some(RespValue::Error("error".to_string()));
        let output = extract_values(input);
        assert_eq!(output.0, "".to_string());
        assert_eq!(output.1, "".to_string());
        assert_eq!(output.2, "".to_string());

        let input = None;
        let output = extract_values(input);
        assert_eq!(output.0, "".to_string());
        assert_eq!(output.1, "".to_string());
        assert_eq!(output.2, "".to_string());
    }

    #[test]
    fn test_extract_id() {
        let input = Some(RespValue::BulkString("an id".as_bytes().to_vec()));
        let output = extract_id(input);
        assert_eq!(output, "an id".to_string());

        let input = None;
        let output = extract_id(input);
        assert_eq!(output, "".to_string());

        let input = Some(RespValue::Error("error".to_string()));
        let output = extract_id(input);
        assert_eq!(output, "".to_string());

        let input = Some(RespValue::Array(vec![RespValue::BulkString(
            "something".as_bytes().to_vec(),
        )]));
        let output = extract_id(input);
        assert_eq!(output, "".to_string());
    }
}
