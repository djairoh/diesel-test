mod models;
mod actions;
mod schema;

use actix_web::{get, middleware, web::{Data, block, Path}, App, Error, HttpResponse, HttpServer};
use diesel::{SqliteConnection, r2d2::{self, ConnectionManager}};
use env_logger;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/songs")]
async fn get_songs(pool: Data<DbPool>) -> Result<HttpResponse, Error> {
    //use web::block to offload Diesel code without blocking server thread
    let songs = block(move || {
        let mut conn = pool.get()?;
        actions::get_songs::get_songs(&mut conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match songs {
        Some(songs) => Ok(HttpResponse::Ok().json(songs)),
        None => Ok(HttpResponse::NotFound().body(format!("No songs were found in the database!")))
    }
}

#[get("/artists/{name}")]
async fn get_songs_by_artist(
    pool: Data<DbPool>,
    artist_name: Path<String>,
) -> Result<HttpResponse, Error> {
    let artist_name = artist_name.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let songs = block(move || {
        let mut conn = pool.get()?;
        actions::get_songs_by_artist::get_songs_by_artist(&mut conn, artist_name)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match songs {
        Some(songs) => Ok(HttpResponse::Ok().json(songs)),
        None => Ok(HttpResponse::NotFound().body("No songs found for this artist!"))
    }
}



#[get("/songs/{name}")]
async fn get_song_by_name(
    pool: Data<DbPool>,
    song_name: Path<String>,
) -> Result<HttpResponse, Error> {
    let song_name = song_name.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let song = block(move || {
        let mut conn = pool.get()?;
        actions::get_song_by_name::get_song_by_name(&mut conn, song_name)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match song {
        Some(song) => Ok(HttpResponse::Ok().json(song)),
        None => Ok(HttpResponse::NotFound().body("No songs found!"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load env variables
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    //set up database connection pool
    //let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    //todo: ^ this no work | v this bad
    let conn_spec = String::from("../tracks.db");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    //start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(get_songs)
            .service(get_song_by_name)
            .service(get_songs_by_artist)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await

}