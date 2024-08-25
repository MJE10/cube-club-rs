use crate::model::color::RubikColor;
use anyhow::anyhow;
use rocket::futures::TryStreamExt;
use sqlx::SqliteConnection;

#[derive(Clone)]
pub struct MosaicDesign {
    pub id: i32,
    pub name: String,
    pub filename: String,
    pub width_pixels: i64,
    pub height_pixels: i64,
    pub pixels: Vec<Vec<RubikColor>>,
}

#[allow(dead_code)]
impl MosaicDesign {
    pub async fn get(db: &mut SqliteConnection, id: i32) -> anyhow::Result<Self> {
        let r = sqlx::query!("SELECT * FROM mosaic_design WHERE id = ?", id)
            .fetch_one(db)
            .await?;
        Ok(Self {
            id,
            name: r.name,
            filename: r.filename,
            width_pixels: r.width_pixels,
            height_pixels: r.height_pixels,
            pixels: serde_json::from_str(&r.pixels)?,
        })
    }

    pub async fn insert(&self, db: &mut SqliteConnection) -> anyhow::Result<i64> {
        let pixels = serde_json::to_string(&self.pixels)?;
        Ok(sqlx::query!(
            "INSERT INTO mosaic_design (name, filename, width_pixels, height_pixels, pixels) VALUES (?, ?, ?, ?, ?) RETURNING id",
            self.name,
            self.filename,
            self.width_pixels,
            self.height_pixels,
            pixels
        ).fetch(db).try_collect::<Vec<_>>().await?.first().ok_or(anyhow!("insert failed"))?.id)
    }

    pub async fn update(&self, db: &mut SqliteConnection) -> anyhow::Result<u64> {
        let pixels = serde_json::to_string(&self.pixels)?;
        Ok(sqlx::query!(
            "UPDATE mosaic_design SET name = ?, filename = ?, width_pixels = ?, height_pixels = ?, pixels = ? WHERE id = ?",
            self.name,
            self.filename,
            self.width_pixels,
            self.height_pixels,
            pixels,
            self.id
        )
        .execute(db)
        .await?.rows_affected())
    }

    pub async fn delete(&self, db: &mut SqliteConnection) -> anyhow::Result<u64> {
        Ok(
            sqlx::query!("DELETE FROM mosaic_design WHERE id = ?", self.id)
                .execute(db)
                .await?
                .rows_affected(),
        )
    }
}
