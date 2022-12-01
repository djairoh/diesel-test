pub mod get_songs;
pub mod get_songs_by_artist;
pub mod get_song_by_name;
pub mod write_song;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;