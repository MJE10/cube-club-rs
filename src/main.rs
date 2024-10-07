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
use crate::login::{checkin, google_callback, google_login, logout, GoogleUserInfo};
use crate::model::base::{error_catcher, Base, Init};
use crate::model::config::Config;
use crate::model::scramble::ScrambleManager;
use crate::mosaic::{
    mosaic_claim, mosaic_get_tiles, mosaic_reset_own_tiles, mosaic_reset_tiles, mosaic_select_page,
    mosaic_user_page, mosaic_viewer, set_design,
};
use crate::timer::{leaderboard_view, timer_base};
use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket_db_pools::Database;
use rocket_dyn_templates::{context, Template};
use rocket_oauth2::OAuth2;

#[derive(Database)]
#[database("cube_club")]
struct CubeClub(sqlx::SqlitePool);

#[get("/")]
async fn index(init: Init) -> Template {
    init.do_(|base| async move { Ok(Template::render("index", context! {base})) })
        .await
}

#[get("/cg")]
async fn cg(init: Init, config: Config) -> Redirect {
    init.do_redirect(|_| async move { Ok(Redirect::to(config.cg_link)) })
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
        .manage(ScrambleManager::new())
        .register("/", catchers![error_catcher])
        .mount("/", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                error,
                cg,
                timer_base,
                leaderboard_view,
                google_login,
                google_callback,
                logout,
                submit_single,
                get_leaderboard,
                checkin,
            ],
        )
        .mount("/admin", routes!(become_user))
        .mount(
            "/mosaic",
            routes![
                mosaic_user_page,
                mosaic_viewer,
                mosaic_select_page,
                set_design,
                mosaic_reset_tiles,
                mosaic_reset_own_tiles,
            ],
        )
        .mount("/api/mosaic", routes![mosaic_claim, mosaic_get_tiles])
}
