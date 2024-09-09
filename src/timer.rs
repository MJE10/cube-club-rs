use crate::model::puzzle::Puzzle;
use crate::model::scramble::{Scramble, ScrambleManager};
use crate::{Base, CubeClub};
use rocket::State;
use rocket_db_pools::Connection;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct TimerProps {
    base: Base,
    event: String,
    scramble: String,
}

#[get("/timer")]
pub async fn timer_base(
    mut db: Connection<CubeClub>,
    scrambles: &State<ScrambleManager>,
    base: Base,
) -> Result<Template, String> {
    let scramble = Scramble::generate(&mut db, scrambles, Puzzle::Three)
        .await
        .map_err(|_| "Scramble generation failed".to_string())?;
    Ok(Template::render(
        "timer",
        TimerProps {
            base,
            event: "3x3 Single".to_string(),
            scramble: scramble.scramble,
        },
    ))
}
