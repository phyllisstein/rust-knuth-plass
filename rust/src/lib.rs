pub mod lines;

use wasm_bindgen::prelude::*;
use lines::graf::Graf;

#[wasm_bindgen]
pub fn break_lines(lines: &str) -> String {
    let mut g = Graf::new(lines.to_string());
    g.parse_nodes();
    let nodes = g.get_nodes();

    let result = format!("{:?}", nodes);
    println!("{}", result);

    result
}
