use actix_web::{get, web, web::ServiceConfig,  HttpResponse};

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

#[get("/1/{num1}/{num2220}")]
async fn cubebits(path: web::Path<(u32, String)>) -> HttpResponse {
    let (num1, num2220) = path.into_inner();
    let num2 = num2220.split('/')
                            .map(|x|x.parse::<u32>().unwrap())
                            .fold(0, |xor, x| xor ^ x);
    HttpResponse::Ok().body((num1 ^ num2).pow(3).to_string())
}