//use crate::model::feed::{FeedItem, FeedItemRequest, FeedItemResponse, FeedItemsResponse};
use crate::model::feed::{FeedItem, FeedItemRequest, FeedItemResponse};
use crate::share::common::AppState;
use actix_redis::{Command, Error as ARError};
use actix_web::{AsyncResponder, Error as AWError, HttpRequest, HttpResponse, Json};
use chrono::Local;
use futures::Future;
use redis_async::resp::RespValue;

pub fn get_item_by_key(
    req: HttpRequest<AppState>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let redis = req.state().redis_addr.clone();
    let key = req.query().get("key").unwrap_or(&"".to_string()).to_owned();
    redis
        .send(Command(resp_array!["HGET", "feeditems", &key]))
        .map_err(AWError::from)
        .and_then(move |res: Result<RespValue, ARError>| match res {
            Ok(RespValue::BulkString(x)) => {
                let f: FeedItem = serde_json::from_slice(&x).unwrap();
                Ok(HttpResponse::Ok().content_type("application/json").json(FeedItemResponse{
                    key: key,
                    title: f.title,
                    body: f.body,
                    time: f.time
                }))
            }
            Ok(RespValue::Nil) => Ok(HttpResponse::NotFound()
                .content_type("text/plain")
                .body("No record found with that key")),
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

    let item = FeedItem { //cloning because we aren't done with feed_item_req yet
        title: feed_item_req.title.clone(),
        body: feed_item_req.body.clone(),
        time: Local::now().to_string(),
    };

    redis
        .send(Command(resp_array![
            "HSET",
            "feeditems",
            &feed_item_req.key,
            &serde_json::to_string(&item).unwrap()
        ]))
        .map_err(AWError::from)
        .and_then(move |res: Result<RespValue, ARError>| match res {
            Ok(_) => {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(FeedItemResponse {
                        key: feed_item_req.key,
                        title: item.title,
                        body: item.body,
                        time: item.time,
                    }))
            }
            _ => Ok(HttpResponse::InternalServerError().finish()),
        })
        .responder()
}

//pub fn get_all_items(
//    req: HttpRequest<AppState>,
//) -> impl Future<Item = HttpResponse, Error = AWError> {
//    let redis = req.state().redis_addr.clone();
//    redis
//        .send(Command(resp_array!["HGETALL", "feeditems"]))
//        .map_err(AWError::from)
//        .and_then(move |res: Result<RespValue, ARError>| match res {
//            Ok(RespValue::Array(x)) => {
//
//                let mut out = FeedItemsResponse{
//                    items: Vec::new(),
//                };
////                for item in x {
////                    if let RespValue::BulkString(i) = item {
////                        dbg!(&i);
////                        //deserialize into struct, add key, append to vec, serialize vec
////                        let mut f: FeedItemResponse = serde_json::from_slice(&i).unwrap();
////                        out.items.push(f);
////                    }
////                }
//                let out: Vec<FeedItemResponse> = x.into_iter().map(|item| match item {
//                    RespValue::BulkString(x) => x,
//                    //convert to FeedItem, collect, throw in a vec, then serde it to a json string
//                    _ => {}
//                }).collect();
//                Ok(HttpResponse::Ok().content_type("application/json").body(out))
//            },
//            _ => {
//                println!("--->{:?}", res);
//                Ok(HttpResponse::InternalServerError().finish())
//            }
//        })
//        .responder()
//}
