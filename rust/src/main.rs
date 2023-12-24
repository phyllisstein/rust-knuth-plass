pub mod lines;

use std::env;
use lines::graf::Graf;

fn main() {
    let text = env::args()
        .nth(1)
        .expect("Please provide a string");

    println!("Text: {}", text);

    let mut graf = Graf::new(text.clone());
    graf.parse_nodes();

    for node in graf.get_nodes() {
        let msg = match node {
            lines::graf::Node::Box(_) => "Box",
            lines::graf::Node::Glue(_) => "Glue",
            lines::graf::Node::Penalty(_) => "Penalty",
        };

        println!("{:?}", msg);
    }
}
