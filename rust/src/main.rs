pub mod lines;

use lines::graf::Graf;
use std::env;

fn main() {
    let text = env::args().nth(1).unwrap_or(
        "Five Reasons Drinking Milk On the Toilet Is Kind Of a Game Changer".to_string(),
    );

    println!("Text: {}", text);

    let mut graf = Graf::new(text);

    println!("{:?}", graf.parse());
}
