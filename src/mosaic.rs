use crate::model::base::{HtmlBase, Init};
use crate::model::config::Config;
use crate::model::mosaic_design::MosaicDesign;
use crate::model::mosaic_tile::MosaicTile;
use crate::Base;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[post("/claim/<row>/<col>")]
pub async fn mosaic_claim(init: Init, row: i64, col: i64) -> Result<Json<bool>, String> {
    init.do_api(|mut base| async move {
        let user = base.require_any_user()?.id;
        let current_assigned = MosaicTile::get_user(base.db(), row, col).await;
        if current_assigned.is_none() {
            MosaicTile::set_user(base.db(), row, col, Some(user)).await?;
            Ok(true)
        } else if current_assigned == Some(user) || base.require_admin_user().is_ok() {
            MosaicTile::set_user(base.db(), row, col, None).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    })
    .await
}

#[get("/tiles")]
pub async fn mosaic_get_tiles(init: Init) -> Result<Json<Vec<MosaicTile>>, String> {
    init.do_api(|mut base| async move {
        base.require_any_user()?;
        Ok(MosaicTile::get_all(base.db()).await?)
    })
    .await
}

#[get("/reset")]
pub async fn mosaic_reset_tiles(init: Init) -> Redirect {
    init.do_redirect(|mut base| async move {
        base.require_admin_user()?;
        MosaicTile::reset_all(base.db()).await?;
        Ok(Redirect::to("/mosaic"))
    })
    .await
}

#[get("/reset-me")]
pub async fn mosaic_reset_own_tiles(init: Init) -> Redirect {
    init.do_redirect(|mut base| async move {
        let user = base.require_any_user()?.id;
        MosaicTile::reset_user(base.db(), user).await?;
        Ok(Redirect::to("/mosaic"))
    })
    .await
}

type MosaicGridArgs = Vec<Vec<Vec<Vec<String>>>>;

#[derive(Serialize)]
struct UserParams {
    base: Base,
    title: String,
    rows: MosaicGridArgs,
    rows2: MosaicGridArgs,
    initial_data: Vec<MosaicTile>,
}

#[get("/")]
pub async fn mosaic_user_page(init: Init) -> Template {
    init.do_(|mut base| async move {
        Ok(Template::render(
            "mosaic/mosaic_user",
            UserParams {
                initial_data: MosaicTile::get_all(base.db()).await?,
                base,
                title: "Mosaic".to_string(),
                rows: (0..10)
                    .map(|_| {
                        (0..10)
                            .map(|_| {
                                (0..3)
                                    .map(|_| (0..3).map(|_| "#ff0000".to_string()).collect())
                                    .collect()
                            })
                            .collect()
                    })
                    .collect(),
                rows2: (0..10)
                    .map(|_| (0..10).map(|_| vec![vec!["#ff0000".to_string()]]).collect())
                    .collect(),
            },
        ))
    })
    .await
}

#[get("/setDesign/<id>")]
pub async fn set_design(init: Init, id: i64) -> Redirect {
    init.do_redirect(|mut base| async move {
        base.require_admin_user()?;

        let mut config = Config::get(base.db()).await?;
        config.mosaic_design_id = id;
        config.update(base.db()).await?;

        MosaicTile::reset_all(base.db()).await?;
        Ok(Redirect::to("/mosaic"))
    })
    .await
}

#[derive(Serialize)]
struct DesignParams {
    base: HtmlBase,
    id: i64,
    rows: MosaicGridArgs,
}

#[derive(Serialize)]
struct SelectParams {
    base: Base,
    title: String,
    designs: Vec<DesignParams>,
}

#[get("/select")]
pub async fn mosaic_select_page(init: Init) -> Template {
    init.do_(|mut base| async move {
        base.require_admin_user()?;

        let ids = MosaicDesign::list(base.db()).await?;
        let mut designs = vec![];
        for id in ids {
            let design = MosaicDesign::get(base.db(), id).await?;
            designs.push(DesignParams {
                base: base.clone(),
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
            base,
            title: "Mosaic Select".to_string(),
            designs,
        };
        Ok(Template::render("mosaic/mosaic_select", params))
    })
    .await
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
            let cube: Vec<Vec<String>> = (0..3)
                .map(|h2| (0..3).map(|w2| colors(h + h2, w + w2)).collect())
                .collect();
            row.push(cube);
        }
        rows.push(row);
    }

    rows
}
