use diesel::SqliteConnection;
use diesel::prelude::*;
use self::models::*;
use backend::*;
use schema::tracks;
use crate::actions::DbError;

pub fn get_songs(conn: &mut SqliteConnection) -> Result<Option<Vec<Song>>, DbError> {
    let songs = tracks::table
        .load::<Song>(conn)
        .optional()?;
    Ok(songs)
}