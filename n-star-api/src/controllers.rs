use actix_web::{post, HttpResponse, Responder};
use serde_json::json;

use crate::github_api::request_github;

#[post("/hook")]
pub async fn hook() -> impl Responder {
    let repo_info = request_github().await;
    let repository_url = &repo_info["items"][0]["html_url"];
    let stargazers_count = &repo_info["items"][0]["stargazers_count"];
    let result = json!({
        "repositoryUrl": repository_url,
        "stars": stargazers_count
    });
    HttpResponse::Ok().json(result)
}