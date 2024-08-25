#[macro_use]
extern crate rocket;

use rocket_db_pools::{Connection, Database};
use sqlx;

#[derive(Database)]
#[database("cube_club")]
struct CubeClub(sqlx::SqlitePool);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
async fn rread(mut db: Connection<CubeClub>, id: i64) -> Option<String> {
    sqlx::query!("SELECT * FROM mosaic_design WHERE id = ?", id)
        .fetch_one(&mut **db)
        .await
        .and_then(|r| Ok(r.name))
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CubeClub::init())
        .mount("/", routes![index, rread])
}
