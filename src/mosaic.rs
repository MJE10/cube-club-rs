use crate::model::mosaic_design::MosaicDesign;
use crate::{App, CubeClub};
use rocket::State;
use rocket_db_pools::Connection;
use sqlx::SqliteConnection;

pub struct Mosaic {
    design_id: i64,
    available: Vec<i64>,
    in_progress: Vec<i64>,
}

impl Default for Mosaic {
    fn default() -> Self {
        Self {
            design_id: 1,
            available: vec![],
            in_progress: vec![],
        }
    }
}

impl Mosaic {
    pub async fn set_all(&mut self) -> anyhow::Result<()> {
        self.available = vec![];
        self.in_progress = vec![];
        Ok(())
    }

    pub async fn clear_all(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        let design = MosaicDesign::get(db, self.design_id).await?;
        self.available = (0..design.height_pixels)
            .map(|h| {
                (0..design.width_pixels)
                    .map(|w| h * design.width_pixels + w)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        Ok(())
    }
}

#[post("/clear")]
pub async fn mosaic_clear(mut db: Connection<CubeClub>, app: &State<App>) -> Result<(), String> {
    app.mosaic
        .write()
        .await
        .clear_all(&mut *db)
        .await
        .map_err(|e| e.to_string())
}

#[post("/reset")]
pub async fn mosaic_reset(app: &State<App>) -> Result<(), String> {
    app.mosaic
        .write()
        .await
        .set_all()
        .await
        .map_err(|e| e.to_string())
}
