use dotenv::dotenv;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let text = warp::path!("texte" / u32)
        .and(warp::path::end())
        .map(|_a| "Unimplemented!");

    let collections = warp::path!("collections")
        .and(warp::path::end())
        .map(|_a| "Unimplemented!");

    let routes = warp::get()
        .and(text_by_id.or(warp::path!("parcours" / u32 / String).map(|_, _| "Unimplemented!")));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
