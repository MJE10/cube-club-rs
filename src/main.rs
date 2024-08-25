mod model;

#[macro_use]
extern crate rocket;

use crate::model::mosaic_design::MosaicDesign;
use rocket::fs::FileServer;
use rocket_db_pools::{Connection, Database};
use sqlx;

#[derive(Database)]
#[database("cube_club")]
struct CubeClub(sqlx::SqlitePool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CubeClub::init())
        .mount("/public", FileServer::from("www/public/"))
        .mount("/", routes![index])
}
