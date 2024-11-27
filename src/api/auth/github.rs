use crate::app_config::AppConfig;
use actix_web::web;
use maud::{html, Markup};
use octocrab::Octocrab;
use serde::Deserialize;
use std::sync::Arc;

pub fn web_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/github")
        .route(
        "/login",
        web::get().to(|app_config: web::Data<Arc<AppConfig>>| async move {
            let client_id = app_config.clone().github_client_id.clone();

            wrap_body(html! {
                a href=(format!("https://github.com/login/oauth/authorize?client_id={client_id}")) {
                    "Login with GitHub"
                }
            })
        }),
    )
        // 实际开发中直接用这个就可以，login作为前端调用的入口
        .route("/callback", web::get().to(|app_config: web::Data<Arc<AppConfig>>,
         web::Query(params): web::Query<CallbackParams>| async move {
        let client_id = app_config.github_client_id.clone();
        let client_secret = app_config.github_client_secret.clone();

        let oauth_client = octocrab::Octocrab::builder()
            .base_uri("https://github.com")
            .unwrap()
            .add_header("accept".parse().unwrap(), "application/json".to_string())
            .build()
            .unwrap();

        let oauth = oauth_client
            .post::<_, serde_json::Value>(
                "/login/oauth/access_token",
                Some(&serde_json::json!({
                "code": params.code,
                "client_id": client_id,
                "client_secret": client_secret,
            })),
            )
            .await
            .unwrap();

        let oauth = serde_json::from_value::<octocrab::auth::OAuth>(oauth.clone())
            .unwrap_or_else(|_| panic!("couldn't parse OAuth credentials from {oauth:?}"));

        let client = Octocrab::builder()
            .user_access_token(oauth.access_token)
            .build()
            .unwrap();

        let user = client.current().user().await.unwrap();

        wrap_body(html! {
        (format!("Hello, {}!", user.login))
    })
    })));
}
#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    code: String,
}
fn wrap_body(markup: Markup) -> Markup {
    html! {
        (maud::DOCTYPE);
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0" ;
                title { "GitHub Login Example" }
            }
            body { (markup) }
        }
    }
}
