use crate::model::base::Init;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;

#[get("/iam_user/<user>")]
pub async fn become_user(user: i64, init: Init, cookies: &CookieJar<'_>) -> Redirect {
    init.do_redirect(|_base| async move {
        #[cfg(not(debug_assertions))]
        _base.require_admin_user()?;
        cookies.remove(Cookie::from("user"));
        cookies.add_private(
            Cookie::build(("user", user.to_string()))
                .same_site(SameSite::Lax)
                .build(),
        );
        Ok(Redirect::to("/"))
    })
    .await
}
