use crate::model::color::RubikColor;
use anyhow::anyhow;
use rocket::futures::TryStreamExt;
use sqlx::SqliteConnection;

#[derive(Clone)]
pub struct MosaicDesign {
    pub id: i64,
    pub name: String,
    pub width_pixels: i64,
    pub height_pixels: i64,
    pub pixels: Vec<Vec<RubikColor>>,
}

#[allow(dead_code)]
impl MosaicDesign {
    pub async fn get(db: &mut SqliteConnection, id: i64) -> anyhow::Result<Self> {
        let r = sqlx::query!("SELECT * FROM mosaic_design WHERE id = ?", id)
            .fetch_one(db)
            .await?;
        Ok(Self {
            id,
            name: r.name,
            width_pixels: r.width_pixels,
            height_pixels: r.height_pixels,
            pixels: Self::deserialize_pixels(&serde_json::from_str(&r.pixels)?)?,
        })
    }

    pub async fn insert(&self, db: &mut SqliteConnection) -> anyhow::Result<i64> {
        let pixels = serde_json::to_string(&self.serialize_pixels())?;
        Ok(sqlx::query!(
            "INSERT INTO mosaic_design (name, width_pixels, height_pixels, pixels) VALUES (?, ?, ?, ?) RETURNING id",
            self.name,
            self.width_pixels,
            self.height_pixels,
            pixels
        ).fetch(db).try_collect::<Vec<_>>().await?.first().ok_or(anyhow!("insert failed"))?.id)
    }

    pub async fn update(&self, db: &mut SqliteConnection) -> anyhow::Result<u64> {
        let pixels = serde_json::to_string(&self.serialize_pixels())?;
        Ok(sqlx::query!(
            "UPDATE mosaic_design SET name = ?, width_pixels = ?, height_pixels = ?, pixels = ? WHERE id = ?",
            self.name,
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

    fn serialize_pixels(&self) -> Vec<String> {
        self.pixels
            .iter()
            .map(|row| row.iter().map(|c| c.to_letter()).collect())
            .collect()
    }

    fn deserialize_pixels(pixels: &Vec<String>) -> anyhow::Result<Vec<Vec<RubikColor>>> {
        pixels
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| {
                        RubikColor::from_letter(c).ok_or_else(|| anyhow!("Invalid character {}", c))
                    })
                    .collect::<anyhow::Result<Vec<RubikColor>>>()
            })
            .collect::<anyhow::Result<Vec<Vec<RubikColor>>>>()
    }
}
