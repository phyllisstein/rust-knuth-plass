pub mod lines;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn break_lines(text: &str) -> String {
    lines::break_lines::break_lines(text)
}
