use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RepoPath {
    repo: String,
}

#[get("/check/github/{repo}")]
pub async fn check_github(path: web::Path<RepoPath>) -> impl Responder {
    let url = format!("https://api.github.com/repos/{}", path.repo);
    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("User-Agent", "infero-ai")
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().body("GitHub repository found.")
            } else {
                HttpResponse::NotFound().body("GitHub repository not found.")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error reaching GitHub API."),
    }
}
