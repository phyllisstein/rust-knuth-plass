pub mod lines;

use lines::graf::Graf;
use lines::nodes::Node;
use std::env;

fn main() {
    let text = env::args().nth(1).expect("Please provide a string");

    println!("Text: {}", text);

    let graf = Graf::new(text.to_string());

    println!("{:?}", graf.to_nodes());
    println!("{:?}", graf.to_breakpoints());
}
