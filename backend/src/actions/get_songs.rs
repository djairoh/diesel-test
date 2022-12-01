use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::actions::DbError;
use crate::db::{models::*, schema::tracks};

pub fn get_songs(conn: &mut SqliteConnection) -> Result<Option<Vec<Song>>, DbError> {
    let songs = tracks::table
        .load::<Song>(conn)
        .optional()?;
    Ok(songs)
}