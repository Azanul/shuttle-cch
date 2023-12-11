use serde_json::{Value, json};
use data_encoding::BASE64;

pub fn decode_base64(input: &str) -> Vec<u8> {
    BASE64.decode(input.as_bytes()).unwrap()
}

pub fn bake_cookies(input_value: Value) -> Value {
    let input = input_value.as_object().unwrap();
    let recipe = input["recipe"].as_object().unwrap();
    let pantry = input["pantry"].as_object().unwrap();
    let mut n_cookies = f64::INFINITY;
    
    for (ingredient, required_quantity) in recipe {
        if let Some(available_quantity) = pantry.get(ingredient) {
            let required_quantity = required_quantity.as_f64().unwrap();
            let available_quantity = available_quantity.as_f64().unwrap();
            
            let quotient = available_quantity / required_quantity;
            n_cookies = n_cookies.min(quotient);
        } else {
            n_cookies = 0.0;
            break
        }
    }

    let mut mut_input = input.clone();
    let pantry = mut_input["pantry"].as_object_mut().unwrap();
    if n_cookies > 0.0 {
        pantry.iter_mut()
            .for_each(|(k, v)| 
                *v = serde_json::Value::from(v.as_i64().unwrap() - n_cookies as i64 * recipe[k].as_i64().unwrap())
            );
    }

    json!({
        "cookies": n_cookies as i64,
        "pantry": pantry,
    })
}