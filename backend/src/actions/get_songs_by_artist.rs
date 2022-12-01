use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::db::{models::*, schema::tracks};
use crate::actions::DbError;


pub fn get_songs_by_artist(conn: &mut SqliteConnection, artist: String) -> Result<Option<Vec<Song>>, DbError> {
    let artist = "%".to_owned() + &artist + "%";
    let songs = tracks::table
        .filter(tracks::artists.like(artist))
        .load::<Song>(conn)
        .optional()?;
    Ok(songs)
}