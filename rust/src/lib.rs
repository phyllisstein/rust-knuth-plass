#![feature(let_chains)]

pub mod lines;

use lines::graf::Graf;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn break_lines(lines: &str) -> String {
    let g = Graf::new(lines.to_string());

    println!("Nodes: {:?}", g.get_nodes());
    println!("Feasible breakpoints: {:?}", g.get_feasible_breakpoints());
    println!("Active breakpoints: {:?}", g.get_active_breakpoints());

    g.parse().get_hyphens()
}
