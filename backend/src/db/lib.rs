use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use chrono::NaiveDate;
use diesel::{Connection, RunQueryDsl};
use crate::db::{models::*, schema::tracks};

//todo: update to be in-line with the actions::* files
/*
pub fn add_song(id: &str, name: &str, popularity: &i32, duration_ms: &i32, artists: &str, id_artists: &str, release_date: &NaiveDate) {
    let conn = &mut establish_connection();
    let new_track = NewSong {
        id,
        name,
        popularity,
        duration_ms,
        artists,
        id_artists,
        release_date,
    };

    diesel::insert_into(tracks::table)
        .values(&new_track)
        .execute(conn)
        .expect("Failed to add new song to table!");
}
*/