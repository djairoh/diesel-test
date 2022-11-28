use self::models::*;
use diesel::prelude::*;
use backend::*;
use scanf::scanf;
use schema::tracks;


fn get_song_by_name(name: &str) -> Vec<Song> {

    let connection = &mut establish_connection();
    tracks::table
        .filter(tracks::name.eq(name))
        .load::<Song>(connection)
        .expect("Error loading tracks")
}

fn main() {
    let mut name: String = String::new();
    scanf!("{}", name);
    let results = get_song_by_name(&name);

    println!("Displaying {} songs", results.len());
    println!("-----------");
    for song in results {
        println!("{}", song.name);
        println!("{}", song.artists);
        println!("-----------");
    }
}