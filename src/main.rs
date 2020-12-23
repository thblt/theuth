#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use theuth::models::*;
use warp::Filter;

pub mod models;

pub fn connect_db() -> PgConnection {
    // @FIXME Don't crash if PgSql is down.
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn prog_notions_responder(
    conn: PgConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    use theuth::models::*;
    use theuth::schema::prog_notions::dsl::*;

    let results = prog_notions
        .load::<ProgNotion>(&conn)
        .expect("Error loading posts");

    Ok(warp::reply::json(&results))
}

pub async fn prog_reperes_responder(
    conn: PgConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    use theuth::models::*;
    use theuth::schema::prog_reperes::dsl::*;

    let results = prog_reperes
        .load::<ProgRepere>(&conn)
        .expect("Error loading posts");

    Ok(warp::reply::json(&results))
}

pub async fn prog_hlp_responder(
    conn: PgConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    use theuth::schema::prog_hlp::dsl::*;

    let results = prog_hlp
        .load::<ProgHLP>(&conn)
        .expect("Error loading posts");

    Ok(warp::reply::json(&results))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // @TODO Test DB connection before starting up.

    let connect_db_filter = warp::any().map(move || connect_db());

    let api_root = warp::path!("api" / "v0");

    let prog_philo = warp::path!("programmes" / "notions")
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and_then(&prog_notions_responder);

    let prog_reperes = warp::path!("programmes" / "reperes")
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and_then(&prog_reperes_responder);

    let prog_hlp = warp::path!("programmes" / "hlp")
        .and(warp::path::end())
        .and(connect_db_filter.clone())
        .and_then(&prog_hlp_responder);

    let texts = warp::path!("textes")
        .and(warp::path::end())
        .map(|| "Unimplemented!");

    let text = warp::path!("textes" / u32)
        .and(warp::path::end())
        .map(|_a| "Unimplemented!");

    let new_text = warp::path!("textes")
        .and(warp::path::end())
        .map(|| "Unimplemented!");

    let update_text = warp::path!("textes")
        .and(warp::path::end())
        .map(|| "Unimplemented!");

    let get_routes = warp::get().and(prog_philo)
        .or(prog_hlp)
        .or(prog_reperes)
        .or(text).or(text);

    let post_routes = warp::post().and(new_text);

    warp::serve(get_routes).run(([127, 0, 0, 1], 3030)).await;
}
