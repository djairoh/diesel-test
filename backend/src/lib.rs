pub mod models;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use chrono::NaiveDate;
use diesel::{Connection, RunQueryDsl};
use self::models::NewSong;
use crate::schema::tracks;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

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