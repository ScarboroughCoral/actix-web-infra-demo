use actix_web::web;

mod auth;

pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(auth::web_config));
}
