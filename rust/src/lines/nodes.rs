use phf::phf_map;

#[derive(Debug, Clone)]
pub enum Node {
    Box {
        width: u32,
    },
    Glue {
        width: u32,
        stretchability: f32,
        shrinkability: f32,
    },
    Penalty {
        width: u32,
        penalty: i32,
        flagged: bool,
    },
    Null {
        grapheme: String,
    },
}

/**
 * 'a' through 'z' are respectively (9, 10, 8, 10, 8, 6, 9, 10, 5, 6, 10, 5, 15,10,
 * 9, 10, 10, 7, 7, 7, 10, 9, 13, 10, 10, 8) units wide and the characters 'C', 'I',
 * and '-' have respective widths of 13, 6, and 6 units. Commas, semicolons, periods,
 * and apostrophes occupy 5 units each. Glue has specifications (w, y, z) = (6, 3, 2)
 * between words, except that it is (6, 4, 2) after a comma, (6, 4, 1) after a
 * semicolon, and (8, 6, 1) after a period. A penalty of 50 has been assessed
 * for every line that ends with a hyphen.
 *      ---Donald E. Knuth, "Breaking Paragraphs Into Lines," in _Digital Typography_, p. 75
 */
pub const LETTER_BOXES: phf::Map<&'static str, Node> = phf_map! {
    "a" => Node::Box { width: 9 },
    "A" => Node::Box { width: 9 },
    "b" => Node::Box { width: 10 },
    "B" => Node::Box { width: 10 },
    "c" => Node::Box { width: 8 },
    "C" => Node::Box { width: 13 },
    "d" => Node::Box { width: 10 },
    "D" => Node::Box { width: 10 },
    "e" => Node::Box { width: 8 },
    "E" => Node::Box { width: 8 },
    "f" => Node::Box { width: 6 },
    "F" => Node::Box { width: 6 },
    "g" => Node::Box { width: 9 },
    "G" => Node::Box { width: 9 },
    "h" => Node::Box { width: 10 },
    "H" => Node::Box { width: 10 },
    "i" => Node::Box { width: 5 },
    "I" => Node::Box { width: 6 },
    "j" => Node::Box { width: 6 },
    "J" => Node::Box { width: 6 },
    "k" => Node::Box { width: 10 },
    "K" => Node::Box { width: 10 },
    "l" => Node::Box { width: 5 },
    "L" => Node::Box { width: 5 },
    "m" => Node::Box { width: 15 },
    "M" => Node::Box { width: 15 },
    "n" => Node::Box { width: 10 },
    "N" => Node::Box { width: 10 },
    "o" => Node::Box { width: 9 },
    "O" => Node::Box { width: 9 },
    "p" => Node::Box { width: 10 },
    "P" => Node::Box { width: 10 },
    "q" => Node::Box { width: 10 },
    "Q" => Node::Box { width: 10 },
    "r" => Node::Box { width: 7 },
    "R" => Node::Box { width: 7 },
    "s" => Node::Box { width: 7 },
    "S" => Node::Box { width: 7 },
    "t" => Node::Box { width: 7 },
    "T" => Node::Box { width: 7 },
    "u" => Node::Box { width: 10 },
    "U" => Node::Box { width: 10 },
    "v" => Node::Box { width: 9 },
    "V" => Node::Box { width: 9 },
    "w" => Node::Box { width: 13 },
    "W" => Node::Box { width: 13 },
    "x" => Node::Box { width: 10 },
    "X" => Node::Box { width: 10 },
    "y" => Node::Box { width: 10 },
    "Y" => Node::Box { width: 10 },
    "z" => Node::Box { width: 8 },
    "Z" => Node::Box { width: 8 },
    "-" => Node::Box { width: 6 },
    "," => Node::Box { width: 5 },
    ";" => Node::Box { width: 5 },
    "." => Node::Box { width: 5 },
    "'" => Node::Box { width: 5 },
};

pub const PUNCTUATION_GLUE: phf::Map<&'static str, Node> = phf_map! {
    " " => Node::Glue { width: 6, stretchability: 3.0, shrinkability: 2.0 },
    "," => Node::Glue { width: 6, stretchability: 4.0, shrinkability: 2.0 },
    ";" => Node::Glue { width: 6, stretchability: 4.0, shrinkability: 1.0 },
    "." => Node::Glue { width: 8, stretchability: 6.0, shrinkability: 1.0 },
    "-" => Node::Penalty { width: 0, penalty: 50, flagged: false },
};
