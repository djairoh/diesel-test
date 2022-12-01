use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::tracks;
use chrono::naive::NaiveDate;


#[derive(Serialize, Deserialize, Queryable)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub popularity: i32,
    pub duration_ms: i32,
    pub artists: String,
    pub id_artists: String,
    pub release_date: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = tracks)]
pub struct NewSong<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub popularity: &'a i32,
    pub duration_ms: &'a i32,
    pub artists: &'a str,
    pub id_artists: &'a str,
    pub release_date: &'a NaiveDate,
}