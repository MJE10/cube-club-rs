use crate::model::base::Init;
use crate::model::puzzle::Puzzle;
use crate::model::scramble::{Scramble, ScrambleManager};
use crate::Base;
use rocket::State;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct TimerProps {
    base: Base,
    event: String,
    scramble: String,
    scramble_id: i64,
}

#[get("/timer")]
pub async fn timer_base(init: Init, scrambles: &State<ScrambleManager>) -> Template {
    init.do_(|mut base| async move {
        base.require_any_user()?;
        let scramble = Scramble::generate(base.db(), scrambles, Puzzle::Three).await?;
        Ok(Template::render(
            "timer",
            TimerProps {
                base,
                event: "3x3 Single".to_string(),
                scramble: scramble.scramble,
                scramble_id: scramble.id,
            },
        ))
    })
    .await
}

#[get("/leaderboard")]
pub async fn leaderboard_view(init: Init) -> Template {
    init.do_(|base| async move { Ok(Template::render("leaderboard", base)) })
        .await
}
