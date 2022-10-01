use api::event::fake;

fn main() {
    let shows = vec![fake::event(), fake::event(), fake::event()];

    for show in shows.iter() {
        println!("{}", show.name);
        println!(
            "{} -- {}",
            show.headliner.as_ref().unwrap().name,
            show.price.as_ref().unwrap(),
        );
        println!(
            "{} at {}",
            show.date.unwrap().format("%B %d"),
            show.start.unwrap().format("%I:00 %P")
        );
        println!()
    }
}
