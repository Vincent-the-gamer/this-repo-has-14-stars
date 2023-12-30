use actix_web::{post, Responder, HttpResponse};
use serde_json::json;

use crate::octocrab;

async fn refresh_star() -> () {
    #[cfg(debug_assertions)]
    {
        tracing::debug!("Getting the token scope…");
        tracing::debug!("Token scope: {:?}", octocrab::get_token_scope().await.unwrap());
    }

    tracing::debug!("Getting the stars…");
    let stars = octocrab::get_stars().await.unwrap();
    tracing::info!("This repo has {stars} stars currently.");

    tracing::debug!("Updating the repo information…");
    octocrab::update_to_stars(stars).await.unwrap();
    tracing::info!("Successfully updated the stars of this repo to {stars}.");
}

#[post("/updateStar")]
pub async fn update_star() -> impl Responder {
    refresh_star().await;
    HttpResponse::Ok().json(json!({
        "msg": "Successfully updated the star!"
    }))
}