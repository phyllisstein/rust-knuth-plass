pub mod lines;

use wasm_bindgen::prelude::*;
use lines::graf::Graf;
use lines::constants::KNUTH_EXAMPLE_GRAF;

#[wasm_bindgen]
pub fn break_lines() -> String {
    let mut g = Graf::new(KNUTH_EXAMPLE_GRAF.to_string());
    g.parse_nodes();
    let nodes = g.get_nodes();

    let result = format!("{:?}", nodes);
    println!("{}", result);

    result
}
