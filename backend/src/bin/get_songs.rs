use self::models::*;
use diesel::prelude::*;
use backend::*;

fn get_songs() -> Vec<Song> {
    use self::schema::tracks::dsl::*;

    let connection = &mut establish_connection();
    tracks
        .load::<Song>(connection)
        .expect("Error loading tracks")
}

fn main() {
    let results = get_songs();
    println!("Displaying {} songs", results.len());
    println!("-----------");
    for song in results {
        println!("{}", song.name);
        println!("{}", song.artists);
        println!("-----------");
    }
}