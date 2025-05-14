use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TwitterPath {
    username: String,
}

#[get("/check/twitter/{username}")]
pub async fn check_twitter(path: web::Path<TwitterPath>) -> impl Responder {
    let username = path.username.clone();
    let twitter_url = format!("https://twitter.com/{}", username);

    let res = reqwest::get(&twitter_url).await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().body("Twitter account exists.")
            } else {
                HttpResponse::NotFound().body("Twitter account not found.")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error reaching Twitter.")
    }
}
