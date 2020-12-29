#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;

pub mod models;
pub mod schema;
pub mod server;

fn main() {
    dotenv().ok();
    server::server_main();
}
