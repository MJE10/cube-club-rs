use crate::model::mosaic_design::MosaicDesign;
use crate::{App, CubeClub};
use rocket::form::validate::Contains;
use rocket::response::Redirect;
use rocket::State;
use rocket_db_pools::Connection;
use rocket_dyn_templates::Template;
use serde::Serialize;
use sqlx::SqliteConnection;

pub struct Mosaic {
    design_id: i64,
    available: Vec<(i64, i64)>,
    in_progress: Vec<(i64, i64)>,
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
            .step_by(3)
            .map(|h| {
                (0..design.width_pixels)
                    .step_by(3)
                    .map(|w| (w / 3, h / 3))
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

#[post("/toggle/<row>/<cell>")]
pub async fn mosaic_toggle(row: i64, cell: i64, app: &State<App>) -> Result<(), String> {
    let mut mosaic = app.mosaic.write().await;
    let id = (row, cell);

    if mosaic.available.contains(id) {
        mosaic.available.retain(|x| *x != id);
    } else {
        mosaic.available.push(id);
    }
    mosaic.in_progress.retain(|x| *x != id);
    Ok(())
}

#[derive(Serialize)]
struct AdminParams {
    title: String,
    rows: Vec<Vec<[[String; 3]; 3]>>,
}

#[post("/mosaic/done/<id>")]
pub async fn mosaic_done(id: i64) {
    Redirect::to("/thanks");
}

#[post("/mosaic/cancel/<id>")]
pub async fn mosaic_cancel(id: i64) {
    Redirect::to("/thanks");
}

#[get("/mosaic/admin")]
pub async fn mosaic_admin_page(
    mut db: Connection<CubeClub>,
    app: &State<App>,
) -> Result<Template, String> {
    let mosaic = app.mosaic.read().await;
    let design = MosaicDesign::get(&mut *db, mosaic.design_id)
        .await
        .map_err(|e| e.to_string())?;

    let mut rows = vec![];

    for h in (0..design.height_pixels).step_by(3) {
        let mut row = vec![];
        for w in (0..design.width_pixels).step_by(3) {
            let mut cube: [[String; 3]; 3] = [0, 1, 2].map(|h2| {
                [0, 1, 2].map(|w2| {
                    design
                        .pixels
                        .get((h + h2) as usize)
                        .and_then(|r| r.get((w + w2) as usize))
                        .map(|c| {
                            if mosaic.available.contains(&(h / 3, w / 3)) {
                                c.darkened_color().display_rgb().to_string()
                            } else {
                                c.solid_color().display_rgb().to_string()
                            }
                        })
                        .unwrap_or_else(|| "#ffffff".to_string())
                })
            });
            row.push(cube);
        }
        rows.push(row);
    }

    let params = AdminParams {
        title: "Mosaic".to_string(),
        rows,
    };

    Ok(Template::render("mosaic/mosaic_admin", params))
}
