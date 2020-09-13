use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use reqwest::Client;

mod apprise;
mod grafana;
mod utils;

async fn notify(
    data: web::Json<grafana::GrafanaPayload>,
    key: web::Path<String>,
) -> impl Responder {
    let payload = apprise::ApprisePayload::from(data.into_inner());
    let client = Client::new()
        .post(apprise::get_apprise_url(&key).expect("URL Parse"))
        .json(&payload);
    let response = client.send().await.expect("Request send");
    return HttpResponse::new(response.status());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/notify/{key}", web::post().to(notify))
    })
    .bind(format!("0.0.0.0:{}", utils::get_port()))?
    .run()
    .await
}
