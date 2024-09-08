mod login;
mod model;
mod mosaic;
mod timer;

#[macro_use]
extern crate rocket;

use crate::login::{google_callback, google_login, logout, GoogleUserInfo};
use crate::model::scramble::ScrambleManager;
use crate::mosaic::{
    mosaic_admin_page, mosaic_cancel, mosaic_clear, mosaic_done, mosaic_reset, mosaic_select_page,
    mosaic_toggle, mosaic_user_page, set_design, Mosaic,
};
use crate::timer::timer_base;
use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket::tokio::sync::RwLock;
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};
use rocket_oauth2::OAuth2;
use serde::Serialize;
use std::sync::Arc;

#[derive(Database)]
#[database("cube_club")]
struct CubeClub(sqlx::SqlitePool);

pub struct App {
    mosaic: Arc<RwLock<Mosaic>>,
}

#[derive(Serialize)]
pub struct Base {
    pub user: Option<i64>,
}

#[get("/")]
async fn index() -> Template {
    Template::render("index", context! {base: Base { user: Some(2) }})
}

#[get("/leaderboard")]
fn leaderboard() -> Template {
    Template::render("leaderboard", context! {base: Base { user: Some(2) }})
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    dotenv().unwrap();

    rocket::build()
        .attach(Template::fairing())
        .attach(CubeClub::init())
        .attach(OAuth2::<GoogleUserInfo>::fairing("google"))
        .manage(App {
            mosaic: Default::default(),
        })
        .manage(ScrambleManager::new())
        .mount("/", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                timer_base,
                leaderboard,
                google_login,
                google_callback,
                logout
            ],
        )
        .mount(
            "/mosaic",
            routes![
                mosaic_admin_page,
                mosaic_user_page,
                mosaic_select_page,
                set_design
            ],
        )
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
