// @generated automatically by Diesel CLI.

diesel::table! {
    tracks (id) {
        id -> Text,
        name -> Text,
        popularity -> Integer,
        duration_ms -> Integer,
        artists -> Text,
        id_artists -> Text,
        release_date -> Date,
    }
}
