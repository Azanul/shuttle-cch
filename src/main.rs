use actix_web::{get, web, web::ServiceConfig,  HttpRequest, HttpResponse, Result};

use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg
        .service(index).service(error)
        .service(cubebits)
        .service(strength_sum).service(winner_summaries)
        .service(elf_count)
        .service(decode_cookie);
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


use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};

#[derive(Clone, Serialize, Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[get("/4/strength")]
async fn strength_sum(reindeers: web::Json<Vec<Reindeer>>) -> HttpResponse {
    HttpResponse::Ok().body(reindeers.iter().map(|x| x.strength).sum::<u32>().to_string())
}

struct ReindeerContestSummary {
    fastest: Option<Reindeer>,
    tallest: Option<Reindeer>,
    magician: Option<Reindeer>,
    consumer: Option<Reindeer>,
}

impl Serialize for ReindeerContestSummary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ReindeerContestSummary", 4)?;

        if let Some(fastest) = &self.fastest {
            state.serialize_field("fastest", &format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name))?;
        }

        if let Some(tallest) = &self.tallest {
            state.serialize_field("tallest", &format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width))?;
        }

        if let Some(magician) = &self.magician {
            state.serialize_field("magician", &format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power))?;
        }

        if let Some(consumer) = &self.consumer {
            state.serialize_field("consumer", &format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food))?;
        }

        state.end()
    }
}

#[get("/4/contest")]
async fn winner_summaries(reindeers: web::Json<Vec<Reindeer>>) -> HttpResponse {
    let summary = ReindeerContestSummary{
        fastest: reindeers.iter().max_by_key(|r| r.speed.to_bits()).cloned(),
        tallest: reindeers.iter().max_by_key(|r| r.height).cloned(),
        magician: reindeers.iter().max_by_key(|r| r.snow_magic_power).cloned(),
        consumer: reindeers.iter().max_by_key(|r| r.strength).cloned(),
    };

    HttpResponse::Ok().json(summary)
}

use serde_json::json;

#[get("/6")]
async fn elf_count(input_str:String) -> HttpResponse {
    let n_elves_on_shelves = input_str.matches("elf on a shelf").count();
    HttpResponse::Ok().json(json!({
        "elf": input_str.matches("elf").count(),
        "elf on a shelf": n_elves_on_shelves,
        "shelf with no elf on it": input_str.matches("shelf").count() - n_elves_on_shelves
    }))
}

use data_encoding::BASE64;
use serde_json::Value;

#[derive(Debug, Serialize)]
struct CookieRecipe(Value);

#[get("/7/decode")]
async fn decode_cookie(req: HttpRequest) -> HttpResponse {
    let cookie = req.headers().get("Cookie").unwrap().to_str().unwrap().get(7..).unwrap();
    println!("{:?}", cookie);
    HttpResponse::Ok().json(String::from_utf8(BASE64.decode(cookie.as_bytes()).unwrap()).unwrap())
}