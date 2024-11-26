use actix_web::{HttpResponse, web};

pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/test")
                    .route(web::get().to(|| async {HttpResponse::Ok().body("auth test")}))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())))
    );
}