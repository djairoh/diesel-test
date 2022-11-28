use self::models::*;
use diesel::prelude::*;
use backend::*;

fn main() {
    use self::schema::tracks::dsl::*;

    let connection = &mut establish_connection();
    let results = tracks
//        .limit(5)
        .load::<Song>(connection)
        .expect("Error loading tracks");

    println!("Displaying {} songs", results.len());
    println!("-----------");
    for song in results {
        println!("{}", song.name);
        println!("{}", song.artists);
        println!("-----------");
    }
}