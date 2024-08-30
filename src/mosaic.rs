use crate::model::mosaic_design::MosaicDesign;
use crate::{App, CubeClub};
use rand::prelude::SliceRandom;
use rocket::form::validate::Contains;
use rocket::response::Redirect;
use rocket::State;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
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

type MosaicGridArgs = Vec<Vec<[[String; 3]; 3]>>;

#[derive(Serialize)]
struct AdminParams {
    title: String,
    rows: MosaicGridArgs,
}

#[derive(Serialize)]
struct UserParams {
    title: String,
    rows: MosaicGridArgs,
    rows2: MosaicGridArgs,
    row: i64,
    cell: i64,
}

#[derive(Serialize)]
struct DesignParams {
    id: i64,
    rows: MosaicGridArgs,
}

#[derive(Serialize)]
struct SelectParams {
    title: String,
    designs: Vec<DesignParams>,
}

#[post("/done/<row>/<cell>")]
pub async fn mosaic_done(row: i64, cell: i64, app: &State<App>) {
    let mut mosaic = app.mosaic.write().await;
    mosaic.in_progress.retain(|x| *x != (row, cell));
}

#[post("/cancel/<row>/<cell>")]
pub async fn mosaic_cancel(row: i64, cell: i64, app: &State<App>) {
    let mut mosaic = app.mosaic.write().await;
    if mosaic.in_progress.contains(&(row, cell)) {
        mosaic.available.push((row, cell));
        mosaic.in_progress.retain(|x| *x != (row, cell));
    }
}

#[get("/admin")]
pub async fn mosaic_admin_page(
    mut db: Connection<CubeClub>,
    app: &State<App>,
) -> Result<Template, String> {
    let mosaic = app.mosaic.read().await;
    let design = MosaicDesign::get(&mut *db, mosaic.design_id)
        .await
        .map_err(|e| e.to_string())?;

    let rows = make_mosaic_params(design.height_pixels, design.width_pixels, |h, w| {
        design
            .pixels
            .get(h as usize)
            .and_then(|r| r.get(w as usize))
            .map(|c| {
                if !mosaic.available.contains(&(h / 3, w / 3)) {
                    c.darkened_color().display_rgb().to_string()
                } else {
                    c.solid_color().display_rgb().to_string()
                }
            })
            .unwrap_or_else(|| "#ffffff".to_string())
    });

    let params = AdminParams {
        title: "Mosaic".to_string(),
        rows,
    };

    Ok(Template::render("mosaic/mosaic_admin", params))
}

#[get("/")]
pub async fn mosaic_user_page(
    mut db: Connection<CubeClub>,
    app: &State<App>,
) -> Result<Template, String> {
    let mut mosaic = app.mosaic.write().await;

    // choose a random available tile
    let id = match mosaic.available.choose(&mut rand::thread_rng()) {
        Some(id) => *id,
        None => {
            return Ok(Template::render("thanks", context! {}));
        }
    };
    mosaic.in_progress.push(id);
    mosaic.available.retain(|x| *x != id);

    let design = MosaicDesign::get(&mut *db, mosaic.design_id)
        .await
        .map_err(|e| e.to_string())?;

    let rows = make_mosaic_params(design.height_pixels, design.width_pixels, |h, w| {
        design
            .pixels
            .get(h as usize)
            .and_then(|r| r.get(w as usize))
            .map(|c| {
                if (h / 3, w / 3) != id {
                    c.darkened_color().display_rgb().to_string()
                } else {
                    c.solid_color().display_rgb().to_string()
                }
            })
            .unwrap_or_else(|| "#ffffff".to_string())
    });

    let params = UserParams {
        title: "Mosaic".to_string(),
        rows,
        rows2: vec![vec![[0, 1, 2].map(|h| {
            [0, 1, 2].map(|w| {
                design
                    .pixels
                    .get((id.0 * 3 + h) as usize)
                    .and_then(|r| r.get((id.1 * 3 + w) as usize))
                    .map(|c| c.solid_color().display_rgb().to_string())
                    .unwrap_or_else(|| "#ffffff".to_string())
            })
        })]],
        row: id.0,
        cell: id.1,
    };

    Ok(Template::render("mosaic/mosaic_user", params))
}

#[get("/setDesign/<id>")]
pub async fn set_design(
    id: i64,
    app: &State<App>,
    mut db: Connection<CubeClub>,
) -> Result<Redirect, String> {
    let mut mosaic = app.mosaic.write().await;
    mosaic.design_id = id;
    mosaic
        .clear_all(&mut *db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Redirect::to("/mosaic/admin"))
}

#[get("/select")]
pub async fn mosaic_select_page(mut db: Connection<CubeClub>) -> Result<Template, String> {
    let db = &mut *db;
    let ids = MosaicDesign::list(db).await.map_err(|e| e.to_string())?;
    let mut designs = vec![];
    for id in ids {
        let design = MosaicDesign::get(db, id).await.map_err(|e| e.to_string())?;
        designs.push(DesignParams {
            id,
            rows: make_mosaic_params(design.height_pixels, design.width_pixels, |h, w| {
                design
                    .pixels
                    .get(h as usize)
                    .and_then(|r| r.get(w as usize))
                    .map(|c| c.solid_color().display_rgb().to_string())
                    .unwrap_or_else(|| "#ffffff".to_string())
            }),
        })
    }

    let params = SelectParams {
        title: "Mosaic Select".to_string(),
        designs,
    };
    Ok(Template::render("mosaic/mosaic_select", params))
}

fn make_mosaic_params(
    height: i64,
    width: i64,
    colors: impl Fn(i64, i64) -> String,
) -> MosaicGridArgs {
    let mut rows = vec![];

    for h in (0..height).step_by(3) {
        let mut row = vec![];
        for w in (0..width).step_by(3) {
            let cube: [[String; 3]; 3] =
                [0, 1, 2].map(|h2| [0, 1, 2].map(|w2| colors(h + h2, w + w2)));
            row.push(cube);
        }
        rows.push(row);
    }

    rows
}
