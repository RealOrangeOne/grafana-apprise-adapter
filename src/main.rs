use actix_web::client::Client;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
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
    req: HttpRequest,
) -> impl Responder {
    let payload = apprise::ApprisePayload::from(data.into_inner());
    let client = Client::default();
    let apprise_url = match apprise::get_apprise_notify_url(&state.apprise_url, &key) {
        Ok(url) => url,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let authorization_header = req.headers().get(header::AUTHORIZATION);
    return match client
        .post(apprise_url.as_str())
        .if_some(authorization_header, |header, builder| {
            builder.set_header(header::AUTHORIZATION, header.clone())
        })
        .send_json(&payload)
        .await
    {
        Ok(response) => HttpResponse::new(response.status()),
        Err(_) => HttpResponse::BadGateway().finish(),
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
