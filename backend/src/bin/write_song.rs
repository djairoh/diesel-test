use chrono::NaiveDate;
use diesel::RunQueryDsl;
use backend::establish_connection;
use scanf::scanf;
use backend::*;

fn main() {
    let mut id: String = String::new();
    let mut name: String = String::new();
    let mut pop: i32 = 0;
    let mut dur: i32 = 0;
    let mut art: String = String::new();
    let mut id_art: String = String::new();
    let mut date: String = String::new();

    if scanf!("{}| {}| {}| {}| {}| {}| {}", id, name, pop, dur, art, id_art, date).is_ok() {
        let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
        let art = "[".to_owned() + &art + "]";
        let id_art = "[".to_owned() + &id_art + "]";
        add_song(&id, &name, &pop, &dur, &art, &id_art, &date);
    }
}