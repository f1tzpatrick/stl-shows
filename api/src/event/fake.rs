use fake::{
    faker::{lorem::en as lorem, name::en as name},
    Fake, Faker,
};
use titlecase::titlecase;
use uuid::Uuid;

use crate::event::model::*;

pub fn date() -> chrono::NaiveDate {
    let date: chrono::NaiveDate = Faker.fake();
    date
}

pub fn time() -> chrono::NaiveTime {
    let time: chrono::NaiveTime = Faker.fake();
    time
}

pub fn event_name() -> String {
    let name: &str = name::FirstName().fake();
    let words: Vec<String> = lorem::Words(1..2).fake();
    let words = titlecase(words.join(" ").as_str());
    format!("Lorem Ipsum: the {} {} Experience", name, words)
}

pub fn venue_name() -> String {
    let name: &str = lorem::Word().fake();
    format!("The Lorem {}", titlecase(name))
}

pub fn performer_name() -> String {
    let name: &str = name::FirstName().fake();
    let word: &str = lorem::Word().fake();
    let words = titlecase(word);
    format!("{} {}", name, words)
}

pub fn url() -> http::Uri {
    let words: Vec<String> = lorem::Words(1..3).fake();
    let words: String = words.join("/");
    let url = format!("https://example.com/{}", words);
    url.parse::<http::Uri>().unwrap()
}

pub fn price() -> String {
    let dollars = (..=180).fake::<usize>();
    let cents = (..100).fake::<u8>();
    format!("${}.{}", dollars, cents)
}

pub fn performer() -> Performer {
    Performer {
        id: Uuid::new_v4(),
        name: performer_name(),
        tags: None,
    }
}

pub fn venue() -> Venue {
    Venue {
        id: Uuid::new_v4(),
        name: venue_name(),
        address: None,
        location: None,
        what3words: None,
    }
}

pub fn event() -> Event {
    Event {
        id: Uuid::new_v4(),
        name: event_name(),
        venue: Some(venue()),
        url: Some(url()),
        price: Some(price()),
        date: Some(date()),
        doors: None,
        start: Some(time()),
        end: None,
        headliner: Some(performer()),
        openers: Some(vec![performer(), performer(), performer()]),
        tags: None,
    }
}
