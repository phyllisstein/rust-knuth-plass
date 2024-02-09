pub mod lines;

use crate::lines::constants::KNUTH_EXAMPLE_GRAF;
use lines::graf::Graf;
use std::env;

fn main() {
    let text = env::args().nth(1).unwrap_or(
        "Five Reasons Drinking Milk On the Toilet Is Kind Of a Game Changer".to_string(),
    );

    println!("Text: {}", String::from(KNUTH_EXAMPLE_GRAF));

    let mut graf = Graf::new(String::from(KNUTH_EXAMPLE_GRAF));

    println!("{:?}", &graf.get_hyphens());
}
