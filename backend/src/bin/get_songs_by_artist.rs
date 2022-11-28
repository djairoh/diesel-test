use self::models::*;
use diesel::prelude::*;
use backend::*;
use scanf::scanf;
use schema::tracks;


fn get_songs_by_artist(artist: &str) -> Vec<Song> {

    let connection = &mut establish_connection();
    tracks::table
        .filter(tracks::artists.like(artist))
        .load::<Song>(connection)
        .expect("Error loading tracks")
}

fn main() {
    let mut artist: String = String::new();
    scanf!("{}", artist);
    let artist = artist.trim();
    let artist= "%".to_owned() + &artist + "%";
    let results = get_songs_by_artist(&artist);

    println!("Displaying {} songs", results.len());
    println!("-----------");
    for song in results {
        println!("{}", song.name);
        println!("{}", song.artists);
        println!("-----------");
    }
}