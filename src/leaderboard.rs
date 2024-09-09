use crate::model::config::Config;
use crate::model::user::User;
use crate::CubeClub;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::Serialize;
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
    let time: f64 = time.parse().map_err(|_| "invalid time format")?;
    let time: i64 = (time * 1000.0) as i64;
    sqlx::query!(
        "INSERT INTO timed_solve (scramble, user, competition, time_ms, penalty, completed_at) \
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

#[derive(Serialize)]
pub struct LeaderboardTile {
    user_id: i64,
    position: i64,
    name: String,
    extra: f64,
}

#[get("/api/leaderboard")]
pub async fn get_leaderboard(
    config: Config,
    mut db: Connection<CubeClub>,
) -> Result<Json<Vec<LeaderboardTile>>, String> {
    let conn: &mut SqliteConnection = &mut db;
    Ok(Json(
        sqlx::query!(
            "WITH ranked_times AS (
                    SELECT
                        u.id,
                        u.given_name,
                        u.family_name,
                        MIN(ts.time_ms) AS lowest_time_ms
                    FROM timed_solve ts
                             JOIN user u ON ts.user = u.id
                    WHERE ts.competition = ?
                    GROUP BY u.given_name, u.family_name
                )
                SELECT
                    ROW_NUMBER() OVER (ORDER BY lowest_time_ms) AS rowcount,
                    id,
                    given_name,
                    family_name,
                    lowest_time_ms
                FROM ranked_times;
    ",
            config.competition_id
        )
        .fetch_all(conn)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter(|r| r.id.is_some() && r.given_name.is_some() && r.family_name.is_some())
        .map(|r| LeaderboardTile {
            user_id: r.id.unwrap(),
            position: r.rowcount,
            name: (r.given_name.unwrap() + " ")
                + &r.family_name
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap_or(' ')
                    .to_string(),
            extra: (r.lowest_time_ms as f64) / 1000.0,
        })
        .collect::<Vec<_>>(),
    ))
}
