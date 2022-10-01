use chrono::NaiveDate;
use chrono::NaiveTime;
use http::Uri;
use uuid::Uuid;

#[derive(Debug)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub venue: Option<Venue>,
    pub url: Option<Uri>,
    pub price: Option<String>,
    pub date: Option<NaiveDate>,
    pub doors: Option<NaiveTime>,
    pub start: Option<NaiveTime>,
    pub end: Option<NaiveTime>,
    pub headliner: Option<Performer>,
    pub openers: Option<Vec<Performer>>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub location: Option<String>,
    pub what3words: Option<String>,
}

#[derive(Debug)]
pub struct Performer {
    pub id: Uuid,
    pub name: String,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}
