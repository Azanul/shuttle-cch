use actix_web::{get, post, web, web::ServiceConfig,  HttpRequest, HttpResponse, Result, FromRequest};
use core::{day1, day4, day4::IntoReindeerContestSummary, day6, day7};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg
        .service(index).service(error)
        .service(cubebits)
        .service(strength_sum).service(winner_summaries)
        .service(elf_count)
        .service(decode_cookie).service(bake_cookies)
        .service(pokemon_weight).service(pokemon_drop);
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

#[get(r"/1{num1220:(\/(\d)+)+}")]
async fn cubebits(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(day1::cubebits(path.into_inner()).to_string())
}

#[post("/4/strength")]
async fn strength_sum(reindeers: web::Json<Vec<day4::Reindeer>>) -> HttpResponse {
    HttpResponse::Ok().body(day4::strength_sum(reindeers.into_inner()).to_string())
}

#[post("/4/contest")]
async fn winner_summaries(reindeers: web::Json<Vec<day4::Reindeer>>) -> HttpResponse {
    HttpResponse::Ok().json(reindeers.into_inner().contest())
}

use serde::{Serialize, Deserialize};

#[post("/6")]
async fn elf_count(input_str:String) -> HttpResponse {
    HttpResponse::Ok().json(day6::elf_counter(input_str))
}

use serde_json::Value;
use actix_utils::future::{ok, Ready};

#[derive(Debug, Serialize)]
struct CookieRecipe(Value);

impl FromRequest for CookieRecipe {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let cookie = req.headers().get("Cookie").unwrap().to_str().unwrap().get(7..).unwrap();
        ok(CookieRecipe(serde_json::from_str::<serde_json::Value>(&String::from_utf8(day7::decode_base64(cookie)).unwrap()).unwrap()))
    }
}

#[get("/7/decode")]
async fn decode_cookie(cookies: CookieRecipe) -> HttpResponse {
    HttpResponse::Ok().json(cookies)
}

#[get("/7/bake")]
async fn bake_cookies(cookies: CookieRecipe) -> HttpResponse {
    HttpResponse::Ok().json(day7::bake_cookies(cookies.0))
}

#[derive(Deserialize)]
struct Pokemon {
    weight: f64
}

#[get("/8/weight/{poke_num}")]
async fn pokemon_weight(pokemon_id: web::Path<u32>) -> HttpResponse {
    let pokemon = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id))
        .await.unwrap()
        .json::<Pokemon>()
        .await.unwrap();

    HttpResponse::Ok().body(format!("{}",  pokemon.weight / 10.0))
}

const GRAVITATIONAL_CONSTANT: f64 = 9.825;

#[get("/8/drop/{poke_num}")]
async fn pokemon_drop(pokemon_id: web::Path<u32>) -> HttpResponse {
    let pokemon = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id))
        .await.unwrap()
        .json::<Pokemon>()
        .await.unwrap();
    println!("Pokemon weight from API: {}", &pokemon.weight);

    HttpResponse::Ok().body(format!("{}",  (pokemon.weight / 10f64) * (20.0 * GRAVITATIONAL_CONSTANT).sqrt()))
}