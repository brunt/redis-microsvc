use std::fmt;

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
    pub title: String,
    pub body: String,
}

//what we want to see returned
#[derive(Serialize, Deserialize)]
pub struct FeedItemResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub time: String,
}

impl fmt::Display for FeedItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}🤔{}🤔{}", self.title, self.body, self.time)
    }
}

impl PartialEq for FeedItem {
    fn eq(&self, other: &FeedItem) -> bool {
        return self.title == other.title && self.body == other.body && self.time == other.time;
    }
}

#[derive(Serialize)]
pub struct FeedItemsResponse {
    pub items: Vec<FeedItemResponse>,
}
