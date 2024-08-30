mod model;
mod mosaic;

#[macro_use]
extern crate rocket;

use crate::mosaic::{
    mosaic_admin_page, mosaic_cancel, mosaic_clear, mosaic_done, mosaic_reset, mosaic_toggle,
    mosaic_user_page, Mosaic,
};
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};
use sqlx;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Database)]
#[database("cube_club")]
struct CubeClub(sqlx::SqlitePool);

pub struct App {
    mosaic: Arc<RwLock<Mosaic>>,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(CubeClub::init())
        .manage(App {
            mosaic: Default::default(),
        })
        .mount("/", routes![index])
        .mount("/mosaic", routes![mosaic_admin_page, mosaic_user_page])
        .mount(
            "/api/mosaic",
            routes![
                mosaic_clear,
                mosaic_reset,
                mosaic_toggle,
                mosaic_cancel,
                mosaic_done
            ],
        )
}
