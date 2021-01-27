#![warn(clippy::pedantic, clippy::nursery)]

//static OWOS: &[&str] = &["OwO", "UwU"];

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn uwuify(input: &str) -> String {
    input
        .trim()
        .replace('l', "w")
        .char_indices()
        .map(|(i, a)| {
            if a == 'r' {
                match input.get(i + 1..i + 2) {
                    Some(t) => {
                        if t.chars().all(char::is_alphabetic) {
                            match input.get(i - 2..i) {
                                Some(y) => {
                                    if y == " w" {
                                        a
                                    } else {
                                        'w'
                                    }
                                }
                                None => a,
                            }
                        } else {
                            a
                        }
                    }
                    None => a,
                }
            } else {
                a
            }
        })
        .collect()
}
