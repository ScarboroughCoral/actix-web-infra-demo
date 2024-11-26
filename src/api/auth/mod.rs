mod github;

use actix_web::web;

pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(github::web_config));
}
