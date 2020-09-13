use actix_web::client::Client;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::process::exit;
use url::Url;
mod apprise;
mod grafana;
mod utils;

#[derive(Clone)]
struct AppState {
    pub apprise_url: Url,
}

async fn notify(
    data: web::Json<grafana::GrafanaPayload>,
    key: web::Path<String>,
    state: web::Data<AppState>,
) -> impl Responder {
    let payload = apprise::ApprisePayload::from(data.into_inner());
    let apprise_url = apprise::get_apprise_notify_url(&state.apprise_url, &key).expect("URL Parse");
    let client = Client::default();
    return match client.post(apprise_url.as_str()).send_json(&payload).await {
        Ok(response) => HttpResponse::new(response.status()),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

async fn health() -> impl Responder {
    return HttpResponse::Ok();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let apprise_url = match apprise::get_apprise_url() {
        Some(h) => h,
        None => {
            log::error!("Invalid apprise host");
            exit(1);
        }
    };

    let app_state = AppState { apprise_url };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(app_state.clone())
            .route("/notify/{key}", web::post().to(notify))
            .route("/health", web::get().to(health))
    })
    .workers(utils::get_workers())
    .bind(format!("0.0.0.0:{}", utils::get_port()))?
    .run()
    .await
}
