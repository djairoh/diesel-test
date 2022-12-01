use diesel::SqliteConnection;
use diesel::prelude::*;
use crate::db::{models::*, schema::tracks};
use crate::actions::DbError;

pub fn get_song_by_name(conn: &mut SqliteConnection, name: String) -> Result<Option<Song>, DbError> {
    let name = "%".to_owned() + &name + "%";
    let song = tracks::table
        .filter(tracks::name.like(name))
        .first::<Song>(conn)
        .optional()?;
    Ok(song)
}