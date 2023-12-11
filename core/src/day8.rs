use serde::Deserialize;

#[derive(Deserialize)]
struct Pokemon {
    weight: f64
}

pub async fn pokemon_weight(pokemon_id: u32) -> f64 {
    let pokemon = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id))
        .await.unwrap()
        .json::<Pokemon>()
        .await.unwrap();
    pokemon.weight / 10f64
}

const GRAVITATIONAL_CONSTANT: f64 = 9.825;

pub async fn pokemon_drop_momentum(pokemon_id: u32, height: f64) -> f64 {
    pokemon_weight(pokemon_id).await * (2f64 * height * GRAVITATIONAL_CONSTANT).sqrt()
}