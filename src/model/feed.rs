//as stored in redis, key is separate
#[derive(Deserialize, Serialize)]
pub struct FeedItem {
    pub title: String,
    pub body: String,
    pub time: String,
}

//what we ask to be stored
#[derive(Deserialize)]
pub struct FeedItemRequest {
    pub key: String,
    pub title: String,
    pub body: String,
}

//what we want to see returned
#[derive(Serialize, Deserialize)]
pub struct FeedItemResponse {
    pub key: String,
    pub title: String,
    pub body: String,
    pub time: String,
}

//#[derive(Serialize)]
//pub struct FeedItemsResponse {
//    pub items: Vec<FeedItemResponse>
//}