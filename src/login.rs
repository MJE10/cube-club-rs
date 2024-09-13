use crate::model::base::Init;
use crate::model::user::User;
use anyhow::Context;
use log::info;
use reqwest::header::AUTHORIZATION;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket_oauth2::{OAuth2, TokenResponse};

/// User information to be retrieved from the Google People API.
#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct GoogleUserInfo {
    sub: String,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("user"));
    Redirect::to("/")
}

#[get("/login/google")]
pub async fn google_login(
    init: Init,
    oauth2: OAuth2<GoogleUserInfo>,
    cookies: &CookieJar<'_>,
) -> Redirect {
    init.do_redirect(|_base| async move { Ok(oauth2.get_redirect(cookies, &["profile"])?) })
        .await
}

#[get("/auth/google")]
pub async fn google_callback(
    init: Init,
    token: TokenResponse<GoogleUserInfo>,
    cookies: &CookieJar<'_>,
) -> Redirect {
    init.do_redirect(|mut base| async move {
        // Use the token to retrieve the user's Google account information.
        let user_info: GoogleUserInfo = reqwest::Client::builder()
            .build()
            .context("failed to build reqwest client")?
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .header(AUTHORIZATION, format!("Bearer {}", token.access_token()))
            .send()
            .await
            .context("failed to complete request")?
            .json()
            .await
            .context("failed to deserialize response")?;

        info!("{} logged in", user_info.name);

        let id = User {
            id: 0,
            sub: user_info.sub,
            full_name: user_info.name,
            given_name: user_info.given_name,
            family_name: user_info.family_name,
        }
        .insert(base.db())
        .await?;

        // Set a private cookie with the user's name, and redirect to the home page.
        cookies.add_private(
            Cookie::build(("user", id.to_string()))
                .same_site(SameSite::Lax)
                .build(),
        );
        Ok(Redirect::to("/"))
    })
    .await
}
