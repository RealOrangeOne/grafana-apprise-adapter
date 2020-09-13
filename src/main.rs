use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

use serde::Deserialize;

mod utils;

#[derive(Deserialize, Debug)]
struct GrafanaPayload {
    title: String,
    message: String,
}

async fn notify(data: web::Json<GrafanaPayload>, key: web::Path<String>) -> impl Responder {
    println!("Data: {:?}", data);
    println!("Key: {:?}", key);
    return HttpResponse::NoContent();
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
