pub mod lines;

use lines::graf::Graf;
use lines::nodes::Node;
use std::env;

fn main() {
    let text = env::args().nth(1).expect("Please provide a string");

    println!("Text: {}", text);

    let graf = Graf::new(text.to_string());

    for node in graf.to_nodes() {
        let msg = match node {
            Node::Box { width } => "Box",
            Node::Glue {
                width,
                stretchability,
                shrinkability,
            } => "Glue",
            Node::Penalty {
                penalty,
                width,
                flagged,
            } => "Penalty",
        };

        println!("{:?}", msg);
    }
}
