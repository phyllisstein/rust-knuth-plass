pub mod lines;

use lines::graf::Graf;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn break_lines(lines: &str) -> String {
    let g = Graf::new(lines.to_string());
    let nodes = g.to_nodes();

    let result = format!("{:?}", nodes);
    println!("{}", result);

    result
}
