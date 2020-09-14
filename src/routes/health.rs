use actix_web::{HttpResponse, Responder};

pub async fn health() -> impl Responder {
    return HttpResponse::Ok();
}
