use crate::model::error::ErrorRecord;
use crate::CubeClub;
use rocket::http::Status;
use rocket::request;
use rocket_db_pools::Connection;
use sqlx::SqliteConnection;

#[allow(dead_code)]
pub struct Config {
    pub competition_id: i64,
    pub cg_link: String,
    pub mosaic_design_id: i64,
}

impl Config {
    pub async fn get(db: &mut SqliteConnection) -> anyhow::Result<Self> {
        let r = sqlx::query!("SELECT competition, cg_link, mosaic_design FROM _config LIMIT 1")
            .fetch_one(db)
            .await?;
        Ok(Self {
            competition_id: r.competition,
            cg_link: r.cg_link,
            mosaic_design_id: r.mosaic_design,
        })
    }

    pub async fn update(&self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE _config SET competition = ?, cg_link = ?, mosaic_design = ?",
            self.competition_id,
            self.cg_link,
            self.mosaic_design_id
        )
        .execute(db)
        .await?;
        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for Config {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<Config, ()> {
        let mut db = request.guard::<Connection<CubeClub>>().await.expect("db");
        match Config::get(&mut db).await {
            Ok(config) => request::Outcome::Success(config),
            Err(e) => {
                ErrorRecord::blind_try_create(&mut db, None, e).await;
                request::Outcome::Error((Status::new(500), ()))
            }
        }
    }
}
