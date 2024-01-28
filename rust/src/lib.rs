pub mod lines;

use lines::graf::Graf;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn break_lines(lines: &str) -> String {
    let mut g = Graf::new(lines.to_string());

    g.get_hyphens()
}
