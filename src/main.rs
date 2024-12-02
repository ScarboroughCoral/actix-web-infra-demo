use crate::app_config::AppConfig;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

mod api;
mod app_config;
mod db;
mod models;
pub mod schema;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let github_client_id = env::var("github_client_id").expect("github_client_id must be set");
    let github_client_secret =
        env::var("github_client_secret").expect("github_client_secret must be set");
    let config = Arc::new(AppConfig {
        github_client_id,
        github_client_secret,
    });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .configure(api::web_config)
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
