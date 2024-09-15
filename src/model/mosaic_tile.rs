use crate::model::color::RubikColor;
use crate::model::config::Config;
use crate::model::mosaic_design::MosaicDesign;
use serde::Serialize;
use sqlx::SqliteConnection;

#[allow(dead_code)]
#[derive(Serialize)]
pub struct MosaicTile {
    pub row: i64,
    pub col: i64,
    pub user: Option<i64>,
    pub colors: [[RubikColor; 3]; 3],
}

#[allow(dead_code)]
impl MosaicTile {
    pub async fn get_user(db: &mut SqliteConnection, row: i64, col: i64) -> Option<i64> {
        sqlx::query!(
            "SELECT assigned_to FROM mosaic_tile WHERE row = ? and col = ?",
            row,
            col
        )
        .fetch_one(db)
        .await
        .ok()
        .map(|r| r.assigned_to)
        .flatten()
    }

    pub async fn set_user(
        db: &mut SqliteConnection,
        row: i64,
        col: i64,
        user: Option<i64>,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE mosaic_tile SET assigned_to = ? WHERE row = ? and col = ?",
            user,
            row,
            col,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn reset_all(db: &mut SqliteConnection) -> anyhow::Result<()> {
        sqlx::query!("UPDATE mosaic_tile SET assigned_to = null WHERE 1=1")
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn reset_user(db: &mut SqliteConnection, user: i64) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE mosaic_tile SET assigned_to = null WHERE assigned_to = ?",
            user
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn get_all(db: &mut SqliteConnection) -> anyhow::Result<Vec<MosaicTile>> {
        let design_id = Config::get(db).await?.mosaic_design_id;
        let design = MosaicDesign::get(db, design_id).await?;
        Ok(
            sqlx::query!("SELECT row, col, assigned_to FROM mosaic_tile")
                .fetch_all(db)
                .await?
                .into_iter()
                .map(|r| MosaicTile {
                    row: r.row,
                    col: r.col,
                    user: r.assigned_to,
                    colors: [0, 1, 2].map(|y| {
                        [0, 1, 2].map(|x| {
                            design
                                .pixels
                                .get(r.row as usize * 3 + y)
                                .and_then(|c| c.get(r.col as usize * 3 + x))
                                .copied()
                                .unwrap_or(RubikColor::White)
                        })
                    }),
                })
                .collect(),
        )
    }
}
