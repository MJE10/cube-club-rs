use crate::CubeClub;
use anyhow::anyhow;
use rocket::futures::TryStreamExt;
use rocket::http::{CookieJar, Status};
use rocket::request;
use rocket_db_pools::Connection;
use serde::Serialize;
use sqlx::SqliteConnection;
use std::ops::DerefMut;

pub const ROLE_ADMIN: i64 = 1;

#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub struct User {
    pub id: i64,
    pub sub: String,
    pub full_name: String,
    pub given_name: String,
    pub family_name: String,
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<User, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");
        let mut db = request.guard::<Connection<CubeClub>>().await.expect("db");

        if let Some(Some(id)) = cookies
            .get_private("user")
            .map(|cookie| cookie.value().parse().ok())
        {
            let res = User::get(&mut db, id).await;
            let conn: &mut SqliteConnection = db.deref_mut();
            match res {
                Ok(user) => {
                    let _ = sqlx::query!(
                        "UPDATE user SET login_at = unixepoch() WHERE id = ?",
                        user.id
                    )
                    .execute(conn)
                    .await;
                    request::Outcome::Success(user)
                }
                _ => request::Outcome::Forward(Status::Unauthorized),
            }
        } else {
            request::Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[allow(dead_code)]
impl User {
    pub async fn get(db: &mut SqliteConnection, id: i64) -> anyhow::Result<User> {
        let r = sqlx::query!(
            "SELECT sub, full_name, given_name, family_name FROM user WHERE id = ?",
            id
        )
        .fetch_one(db)
        .await
        .map_err(|e| anyhow!("User not found: {e}"))?;
        Ok(Self {
            id,
            sub: r.sub,
            full_name: r.full_name,
            given_name: r.given_name,
            family_name: r.family_name,
        })
    }

    pub async fn get_by_sub(db: &mut SqliteConnection, sub: &str) -> anyhow::Result<i64> {
        let r = sqlx::query!("SELECT id FROM user WHERE sub = ? LIMIT 1", sub)
            .fetch_one(db)
            .await?;
        Ok(r.id)
    }

    pub async fn insert(&mut self, db: &mut SqliteConnection) -> anyhow::Result<i64> {
        // first check if the sub already exists
        if let Ok(id) = Self::get_by_sub(db, &self.sub).await {
            self.id = id;
            return Ok(id);
        }
        // if it doesn't exist, insert it
        let id = sqlx::query!(
            "INSERT INTO user (sub, full_name, given_name, family_name, created_at, login_at) VALUES (?, ?, ?, ?, unixepoch(), unixepoch()) RETURNING id",
            self.sub,
            self.full_name,
            self.given_name,
            self.family_name,
        )
        .fetch(db)
        .try_collect::<Vec<_>>()
        .await?
        .first()
        .ok_or(anyhow!("insert failed"))?
        .id;
        self.id = id;
        Ok(id)
    }

    pub async fn has_role(&self, db: &mut SqliteConnection, role: i64) -> anyhow::Result<bool> {
        Ok(sqlx::query!("SELECT COUNT(*) has_role FROM user u JOIN user_role ur on u.id = ur.user WHERE u.id = ? AND ur.id = ?",
            self.id, role).fetch_one(db).await?.has_role > 0)
    }

    pub async fn is_admin(&self, db: &mut SqliteConnection) -> anyhow::Result<bool> {
        self.has_role(db, ROLE_ADMIN).await
    }
}
