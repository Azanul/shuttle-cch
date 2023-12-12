use actix_web::{get, post, web, web::ServiceConfig,  HttpRequest, HttpResponse, Result, FromRequest, Responder, cookie::time::Instant};
use actix_files::NamedFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use image::{GenericImageView, Rgba};
use actix_utils::future::{ok, Ready};
use core::{day1, day4, day4::IntoReindeerContestSummary, day6, day7, day8};
use std::{io::BufReader, collections::HashMap};
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::Mutex;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_state = web::Data::new(AppState {
        stopwatch: Mutex::new(HashMap::new()),
    });

    let config = move |cfg: &mut ServiceConfig| {
        cfg
        .app_data(app_state.clone())
        .service(index).service(error)
        .service(cubebits)
        .service(strength_sum).service(winner_summaries)
        .service(elf_count)
        .service(decode_cookie).service(bake_cookies)
        .service(pokemon_weight).service(pokemon_drop)
        .service(serve_image).service(magical_pixels_count)
        .service(save_id).service(load_id).service(ulids_to_uuids);
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

#[get(r"/1{num1220:(\/(-?\d)+)+}")]
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

#[post("/6")]
async fn elf_count(input_str:String) -> HttpResponse {
    HttpResponse::Ok().json(day6::elf_counter(input_str))
}

use serde::Serialize;
use serde_json::Value;

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

#[get("/8/weight/{poke_num}")]
async fn pokemon_weight(pokemon_id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{}",  day8::pokemon_weight(pokemon_id.into_inner()).await))
}

#[get("/8/drop/{poke_num}")]
async fn pokemon_drop(pokemon_id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{}", day8::pokemon_drop_momentum(pokemon_id.into_inner(), 10f64).await))
}

#[get("/11/assets/{file_name}")]
async fn serve_image(file_name: web::Path<String>) -> impl Responder {
    NamedFile::open(format!("assets/{}", file_name))
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "image")]
    image: TempFile,
}

#[post("/11/red_pixels")]
async fn magical_pixels_count(MultipartForm(form): MultipartForm<UploadForm>) -> HttpResponse {
    let image = image::load(BufReader::new(&form.image.file), image::ImageFormat::Png).unwrap();

    let mut count = 0;
    for (_, _, Rgba([r, g, b, _])) in image.pixels() {
        if r > b.saturating_add(g) { count += 1 }
    }

    HttpResponse::Ok().body(count.to_string())
}

struct AppState {
    stopwatch: Mutex<HashMap<String, Instant>>,
}

#[post("/12/save/{id}")]
async fn save_id(id: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let id = id.into_inner();
    let mut stopwatch = state.stopwatch.lock().unwrap();
    stopwatch.insert(id, Instant::now());
    HttpResponse::Ok()
}

#[get("/12/load/{id}")]
async fn load_id(id: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let id = id.into_inner();
    let stopwatch = state.stopwatch.lock().unwrap();
    HttpResponse::Ok().body(format!("{}", Instant::elapsed(*stopwatch.get(&id).unwrap()).whole_seconds()))
}

use ulid::Ulid;
use uuid::Uuid;

#[post("/12/ulids")]
async fn ulids_to_uuids(data: web::Json<Vec<String>>) -> impl Responder {
    let uuids: Vec<Uuid> = data
        .iter().rev()
        .filter_map(|ulid_str| Ulid::from_string(ulid_str).ok())
        .map(|ulid| Uuid::from_bytes(ulid.to_bytes()))
        .collect();

    HttpResponse::Ok().json(uuids)
}