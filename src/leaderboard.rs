use crate::model::config::Config;
use crate::model::user::User;
use crate::CubeClub;
use rocket_db_pools::Connection;
use sqlx::SqliteConnection;

#[post("/api/submit/<scramble>/<time>")]
pub async fn submit_single(
    scramble: i64,
    time: String,
    config: Config,
    user: User,
    mut db: Connection<CubeClub>,
) -> Result<(), String> {
    let conn: &mut SqliteConnection = &mut db;
    sqlx::query!(
        "INSERT INTO timed_solve (scramble, user, competition, time, penalty, completed_at) \
    VALUES (?, ?, ?, ?, '', unixepoch());",
        scramble,
        user.id,
        config.competition_id,
        time
    )
    .execute(conn)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}
