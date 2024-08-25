use crate::models::MosaicDesign;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod models;
mod schema;

fn main() {
    use crate::schema::mosaic_design::dsl::*;

    println!("Hello, world!");

    let conn = &mut establish_connection();
    let results = mosaic_design
        .filter(width_pixels.eq(30).and(height_pixels.eq(30)))
        .select(MosaicDesign::as_select())
        .load(conn)
        .expect("Error loading");

    for result in results {
        println!("{}", result.name);
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
