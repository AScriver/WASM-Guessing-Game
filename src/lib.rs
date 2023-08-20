extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::cmp::Ordering;
use rand::Rng;

#[wasm_bindgen]
pub struct Game {
    secret_number: i32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        Game { secret_number }
    }

    pub fn guess(&self, number: i32) -> String {
        match number.cmp(&self.secret_number) {
            Ordering::Less => "Too small!".to_string(),
            Ordering::Greater => "Too big!".to_string(),
            Ordering::Equal => "You win!".to_string(),
        }
    }
}
