mod leaderboard;
mod login;
mod model;
mod mosaic;
mod timer;

#[macro_use]
extern crate rocket;

use crate::leaderboard::submit_single;
use crate::login::{google_callback, google_login, logout, GoogleUserInfo};
use crate::model::config::Config;
use crate::model::scramble::ScrambleManager;
use crate::model::user::User;
use crate::mosaic::{
    mosaic_admin_page, mosaic_cancel, mosaic_clear, mosaic_done, mosaic_reset, mosaic_select_page,
    mosaic_toggle, mosaic_user_page, set_design, Mosaic,
};
use crate::timer::timer_base;
use dotenvy::dotenv;
use rocket::fs::FileServer;
use rocket::request;
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

#[derive(Clone, Serialize)]
pub struct Base {
    pub user: Option<i64>,
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for Base {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<Base, ()> {
        let user = match request.guard::<User>().await {
            request::Outcome::Success(user) => Some(user.id),
            _ => None,
        };
        request::Outcome::Success(Base { user })
    }
}

#[get("/")]
async fn index(base: Base, config: Config) -> Result<Template, String> {
    Ok(Template::render(
        "index",
        context! {base, cg_link: config.cg_link},
    ))
}

#[get("/leaderboard")]
fn leaderboard_view(base: Base) -> Template {
    Template::render("leaderboard", context! {base})
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
                leaderboard_view,
                google_login,
                google_callback,
                logout,
                submit_single,
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
