use actix_web::{get, web::ServiceConfig,  HttpResponse};

use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index).service(error);
    };

    Ok(config.into())
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/-1/error")]
async fn error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}
