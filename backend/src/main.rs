use actix_web::{web, App, HttpServer};
use infero_backend::api::{github::check_github, twitter::check_twitter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/check/github/{repo}", web::get().to(check_github))
            .route("/check/twitter/{username}", web::get().to(check_twitter))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
