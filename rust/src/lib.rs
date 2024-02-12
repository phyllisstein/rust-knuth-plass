pub mod lines;

use error_chain::error_chain;
use std::io::Read;
use lines::graf::Graf;
use wasm_bindgen::prelude::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[wasm_bindgen]
pub fn break_lines(lines: &str) -> String {
    let mut g = Graf::new(lines.to_string());
    let hyphens = g.get_hyphens();

    let res = reqwest::blocking::get("http://httpbin.org/get").unwrap();

    res.text().unwrap()
}
