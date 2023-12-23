use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn break_lines(text: &str) -> String {
    let text_string = String::from(text);
    let mut words = vec![];

    for word in text_string.split_whitespace() {
        let word_string = String::from(word);
        let word_length = word_string.len();

        if word_length <= 3 {
            words.push(word_string);
            continue;
        }

        let (first, last) = word_string.split_at(word_length / 2);
        let hyphen_word = format!("{}-{}", first, last);
        words.push(hyphen_word);
    }

    let result = words.join(" ");
    let result = format!("<p>{}</p>", result);

    result
}
