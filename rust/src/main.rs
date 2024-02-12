pub mod lines;

use error_chain::error_chain;
use lines::graf::Graf;
use std::env;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() {
    let text = env::args().nth(1).unwrap_or(
        "Five Reasons Drinking Milk On the Toilet Is Kind Of a Game Changer".to_string(),
    );

    println!("Text: {}", String::from(&text));

    let mut graf = Graf::new(String::from(&text));

    println!("{:?}", graf.get_hyphens());

    let res = reqwest::blocking::get("http://httpbin.org/get").unwrap();

    let body = res.text().unwrap();
    println!("Body: {}", body);
}
