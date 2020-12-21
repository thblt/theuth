#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use theuth::models::*;
use warp::Filter;

pub mod models;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
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

    let db_connect = warp::any().map(move || establish_connection());

    let api_root = warp::path!("api" / "v0");

    let prog_philo = warp::path!("programmes" / "notions")
        .and(warp::path::end())
        .and(db_connect.clone())
        .and_then(&prog_notions_responder);

    let prog_reperes = warp::path!("programmes" / "reperes")
        .and(warp::path::end())
        .and(db_connect.clone())
        .and_then(&prog_reperes_responder);

    let prog_hlp = warp::path!("programmes" / "hlp")
        .and(warp::path::end())
        .and(db_connect.clone())
        .and_then(&prog_hlp_responder);

    let text = warp::path!("texte" / u32)
        .and(warp::path::end())
        .map(|_a| "Unimplemented!");

    let collections = warp::path!("collections")
        .and(warp::path::end())
        .map(|| "Unimplemented!");

    let routes = warp::get().and(prog_philo)
        .or(prog_hlp)
        .or(prog_reperes)
        .or(text).or(collections);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
