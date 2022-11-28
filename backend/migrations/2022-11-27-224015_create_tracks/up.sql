-- Your SQL goes here


CREATE TABLE tracks (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    popularity INTEGER NOT NULL DEFAULT 0,
    duration_ms INTEGER NOT NULL,
    artists TEXT NOT NULL DEFAULT "[]",
    id_artists TEXT NOT NULL DEFAULT "[]",
    release_date DATE NOT NULL DEFAULT NOW
)