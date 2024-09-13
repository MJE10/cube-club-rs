mod admin;
mod leaderboard;
mod login;
mod model;
mod mosaic;
mod timer;

#[macro_use]
extern crate rocket;

use crate::admin::become_user;
use crate::leaderboard::{get_leaderboard, submit_single};
use crate::login::{google_callback, google_login, logout, GoogleUserInfo};
use crate::model::base::{error_catcher, Base, Init};
use crate::model::config::Config;
use crate::model::scramble::ScrambleManager;
use crate::mosaic::{
    mosaic_admin_page, mosaic_cancel, mosaic_clear, mosaic_done, mosaic_reset, mosaic_select_page,
    mosaic_toggle, mosaic_user_page, set_design, Mosaic,
};
use crate::timer::{leaderboard_view, timer_base};
use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket::tokio::sync::RwLock;
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};
use rocket_oauth2::OAuth2;
use std::sync::Arc;

#[derive(Database)]
#[database("cube_club")]
struct CubeClub(sqlx::SqlitePool);

pub struct App {
    mosaic: Arc<RwLock<Mosaic>>,
}

#[get("/")]
async fn index(init: Init, config: Config) -> Template {
    init.do_(|base| async move {
        Ok(Template::render(
            "index",
            context! {base, cg_link: config.cg_link},
        ))
    })
    .await
}

#[get("/error?<code>")]
async fn error(init: Init, code: Option<String>) -> Template {
    init.do_(|base| async move {
        Ok(Template::render(
            "basic/error",
            context! {base, error_code: code},
        ))
    })
    .await
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    dotenv().unwrap();

    rocket::build()
        .attach(Template::fairing())
        .attach(OAuth2::<GoogleUserInfo>::fairing("google"))
        .attach(CubeClub::init())
        .manage(App {
            mosaic: Default::default(),
        })
        .manage(ScrambleManager::new())
        .register("/", catchers![error_catcher])
        .mount("/", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                error,
                timer_base,
                leaderboard_view,
                google_login,
                google_callback,
                logout,
                submit_single,
                get_leaderboard
            ],
        )
        .mount("/admin", routes!(become_user))
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
