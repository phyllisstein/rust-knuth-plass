pub mod lines;

use lines::graf::{Graf, Node};
use std::env;

fn main() {
    let text = env::args().nth(1).expect("Please provide a string");

    println!("Text: {}", text);

    let mut graf = Graf::new(text.clone());
    graf.parse_nodes();

    for node in graf.get_nodes() {
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
