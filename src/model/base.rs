use crate::model::error::ErrorRecord;
use crate::model::user::User;
use crate::CubeClub;
use anyhow::anyhow;
use rocket::response::Redirect;
use rocket::serde::Serialize;
use rocket::{request, Request};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use serde::Serializer;
use sqlx::SqliteConnection;
use std::future::Future;
use std::ops::{Deref, DerefMut};

#[catch(default)]
pub async fn error_catcher(request: &Request<'_>) -> Template {
    let (user, base) = request
        .guard::<Init>()
        .await
        .succeeded()
        .map(|init| (init.base.user.clone(), init.base.clone()))
        .unwrap_or((None, HtmlBase::default()));

    let mut db = request.guard::<Connection<CubeClub>>().await.expect("db");
    ErrorRecord::blind_try_create(&mut db, user, anyhow!("404: {}", request.uri())).await;

    Template::render("basic/error", context! {base, error_code: ""})
}

pub struct Init {
    conn: Connection<CubeClub>,
    base: Base,
}

pub struct Base {
    conn: Connection<CubeClub>,
    base: HtmlBase,
}

#[derive(Clone, Default, Serialize)]
pub struct HtmlBase {
    pub user: Option<User>,
    pub is_admin: bool,
}

impl Deref for Base {
    type Target = HtmlBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Base {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Serialize for Base {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.base.serialize(serializer)
    }
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for Init {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Init, ()> {
        let user = request.guard::<User>().await.succeeded();
        let mut db = request.guard::<Connection<CubeClub>>().await.expect("db");
        let db2 = request.guard::<Connection<CubeClub>>().await.expect("db");
        let is_admin = if let Some(user) = &user {
            user.is_admin(&mut db).await.unwrap_or(false)
        } else {
            false
        };
        request::Outcome::Success(Init {
            conn: db,
            base: Base {
                conn: db2,
                base: HtmlBase { user, is_admin },
            },
        })
    }
}

const NOT_LOGGED_IN: &str = "NOT_LOGGED_IN";
const NOT_ADMIN: &str = "NOT_ADMIN";

impl Init {
    async fn do_internal<T, F: FnOnce(Base) -> Fut, Fut: Future<Output = anyhow::Result<T>>>(
        mut self,
        stuff: F,
    ) -> Result<T, Option<String>> {
        let base = self.base.clone();
        match stuff(self.base).await {
            Ok(t) => Ok(t),
            Err(e) => {
                let s = e.to_string();

                // I don't need to know about every time the users visit the timer page without logging in first
                if s == NOT_LOGGED_IN || s == NOT_ADMIN {
                    return Err(Some(s));
                }

                let hash = ErrorRecord::hash(&e);
                ErrorRecord::blind_try_create(&mut self.conn, base.user.clone(), e).await;

                Err(Some(format!("{hash:05}")))
            }
        }
    }

    pub async fn do_<F: FnOnce(Base) -> Fut, Fut: Future<Output = anyhow::Result<Template>>>(
        self,
        stuff: F,
    ) -> Template {
        let base = self.base.clone();

        self.do_internal(stuff).await.unwrap_or_else(|code| {
            Template::render("basic/error", context! {base, error_code: code})
        })
    }

    pub async fn do_redirect<
        F: FnOnce(Base) -> Fut,
        Fut: Future<Output = anyhow::Result<Redirect>>,
    >(
        self,
        stuff: F,
    ) -> Redirect {
        self.do_internal(stuff).await.unwrap_or_else(|code| {
            Redirect::to(format!(
                "/error{}",
                code.map(|code| format!("?code={code}"))
                    .unwrap_or("".to_string())
            ))
        })
    }
}

impl Base {
    pub fn db(&mut self) -> &mut SqliteConnection {
        &mut self.conn
    }

    pub fn require_any_user(&self) -> anyhow::Result<User> {
        if let Some(user) = &self.user {
            Ok(user.clone())
        } else {
            Err(anyhow!(NOT_LOGGED_IN))
        }
    }

    pub fn require_admin_user(&self) -> anyhow::Result<User> {
        if let Some(user) = &self.user {
            if self.is_admin {
                Ok(user.clone())
            } else {
                Err(anyhow!(NOT_ADMIN))
            }
        } else {
            Err(anyhow!(NOT_LOGGED_IN))
        }
    }
}
