use phf::phf_map;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum Node {
    Box(Box),
    Glue(Glue),
    Penalty(Penalty),
}

#[derive(Debug)]
pub struct Box {
    pub width: u32,
}

#[derive(Debug)]
pub struct Glue {
    pub width: u32,
    pub stretchability: f32,
    pub shrinkability: f32,
}

#[derive(Debug)]
pub struct Penalty {
    pub width: u32,
    pub penalty: i32,
    pub flagged: bool,
}

#[derive(Debug)]
pub struct Breakpoint {
    pub position: i16,
    pub predecessor: i16,
    pub total_width: u32,
    pub stretch: f32,
    pub shrink: f32,
    pub ratio: f32,
    pub line_number: i16,
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
pub const LETTER_WIDTHS: phf::Map<&'static str, Box> = phf_map! {
    "a" => Box { width: 9 },
    "A" => Box { width: 9 },
    "b" => Box { width: 10 },
    "B" => Box { width: 10 },
    "c" => Box { width: 8 },
    "C" => Box { width: 13 },
    "d" => Box { width: 10 },
    "D" => Box { width: 10 },
    "e" => Box { width: 8 },
    "E" => Box { width: 8 },
    "f" => Box { width: 6 },
    "F" => Box { width: 6 },
    "g" => Box { width: 9 },
    "G" => Box { width: 9 },
    "h" => Box { width: 10 },
    "H" => Box { width: 10 },
    "i" => Box { width: 5 },
    "I" => Box { width: 6 },
    "j" => Box { width: 6 },
    "J" => Box { width: 6 },
    "k" => Box { width: 10 },
    "K" => Box { width: 10 },
    "l" => Box { width: 5 },
    "L" => Box { width: 5 },
    "m" => Box { width: 15 },
    "M" => Box { width: 15 },
    "n" => Box { width: 10 },
    "N" => Box { width: 10 },
    "o" => Box { width: 9 },
    "O" => Box { width: 9 },
    "p" => Box { width: 10 },
    "P" => Box { width: 10 },
    "q" => Box { width: 10 },
    "Q" => Box { width: 10 },
    "r" => Box { width: 7 },
    "R" => Box { width: 7 },
    "s" => Box { width: 7 },
    "S" => Box { width: 7 },
    "t" => Box { width: 7 },
    "T" => Box { width: 7 },
    "u" => Box { width: 10 },
    "U" => Box { width: 10 },
    "v" => Box { width: 9 },
    "V" => Box { width: 9 },
    "w" => Box { width: 13 },
    "W" => Box { width: 13 },
    "x" => Box { width: 10 },
    "X" => Box { width: 10 },
    "y" => Box { width: 10 },
    "Y" => Box { width: 10 },
    "z" => Box { width: 8 },
    "Z" => Box { width: 8 },
    "-" => Box { width: 6 },
    "," => Box { width: 5 },
    ";" => Box { width: 5 },
    "." => Box { width: 5 },
    "'" => Box { width: 5 },
};

pub const EMPTY_GLUE: Glue = Glue {
    width: 0,
    stretchability: 0.0,
    shrinkability: 0.0,
};

pub const WORD_GLUE: Glue = Glue {
    width: 6,
    stretchability: 3.0,
    shrinkability: 2.0,
};

pub const COMMA_GLUE: Glue = Glue {
    width: 6,
    stretchability: 4.0,
    shrinkability: 2.0,
};

pub const SEMICOLON_GLUE: Glue = Glue {
    width: 6,
    stretchability: 4.0,
    shrinkability: 1.0,
};

pub const PERIOD_GLUE: Glue = Glue {
    width: 8,
    stretchability: 6.0,
    shrinkability: 1.0,
};

pub const HYPHEN_PENALTY: Penalty = Penalty {
    width: 0,
    penalty: 50,
    flagged: false,
};

#[derive(Debug)]
pub struct Graf {
    plain_text: String,
    nodes: Vec<Node>,
    feasible_breakpoints: Vec<i16>,
    active_breakpoints: Vec<Breakpoint>,
}

impl Graf {
    pub fn new(plain_text: String) -> Graf {
        Graf {
            plain_text,
            nodes: vec![],
            feasible_breakpoints: vec![],
            active_breakpoints: vec![],
        }
    }

    pub fn parse_nodes(&mut self) {
        for (_index, grapheme) in self.plain_text.graphemes(true).enumerate() {
            let node = match LETTER_WIDTHS.get(grapheme) {
                Some(letter) => Node::Box(Box {
                    width: letter.width,
                }),
                None => continue,
            };

            self.nodes.push(node);

            let glue = match grapheme {
                " " => WORD_GLUE,
                "," => COMMA_GLUE,
                ";" => SEMICOLON_GLUE,
                "." => PERIOD_GLUE,
                _ => continue,
            };

            self.nodes.push(Node::Glue(glue));
        }
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn to_vec(&self) -> Vec<&Node> {
        self.nodes.iter().collect()
    }
}
