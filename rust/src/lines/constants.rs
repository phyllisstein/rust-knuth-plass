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
pub const LETTER_WIDTHS: [(&str, Box); 33] = [
    ("a", Box { width: 9 }),
    ("b", Box { width: 10 }),
    ("c", Box { width: 8 }),
    ("d", Box { width: 10 }),
    ("e", Box { width: 8 }),
    ("f", Box { width: 6 }),
    ("g", Box { width: 9 }),
    ("h", Box { width: 10 }),
    ("i", Box { width: 5 }),
    ("j", Box { width: 6 }),
    ("k", Box { width: 10 }),
    ("l", Box { width: 5 }),
    ("m", Box { width: 15 }),
    ("n", Box { width: 10 }),
    ("o", Box { width: 9 }),
    ("p", Box { width: 10 }),
    ("q", Box { width: 10 }),
    ("r", Box { width: 7 }),
    ("s", Box { width: 7 }),
    ("t", Box { width: 7 }),
    ("u", Box { width: 10 }),
    ("v", Box { width: 9 }),
    ("w", Box { width: 13 }),
    ("x", Box { width: 10 }),
    ("y", Box { width: 10 }),
    ("z", Box { width: 8 }),
    ("C", Box { width: 13 }),
    ("I", Box { width: 6 }),
    ("-", Box { width: 6 }),
    (",", Box { width: 5 }),
    (";", Box { width: 5 }),
    (".", Box { width: 5 }),
    ("'", Box { width: 5 }),
];

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

pub const GRAFS: [&str; 3] = [
    r###"
        There's been a lot of lying in this family. And a lot of love! More
        lies. It's a jetpack, Michael. What could possibly go wrong? However,
        she mistook the drowsy eye alcohol warning for a winking eye alcohol
        suggestion. I want to cry so bad, but I don't think I can spare the
        moisture. Everybody dance... NOW.
    "###,
    r###"
        I mean, it's one banana, Michael. What could it cost, ten dollars? And
        THAT'S why you always leave a note. And I am rock steady. No more
        dizzies.
    "###,
    r###"
        I think that's one of Mom's little fibs, you know, like I'll sacrifice
        anything for my children. After all, why should you go to jail for a
        crime somebody else noticed?
    "###,
];

pub const KNUTH_EXAMPLE_GRAF: &str = r###"
    In olden times when wishing still helped one, there lived a king whose daughters
    were all beautiful; and the youngest was so beautiul that the sun itself, which
    has seen so much, was astonished whenever it shone in her face. Close by the
    king's castle lay a great dark forest, and under an old lime-tree in the forest
    was a well, and when the day was very warm, the king's child went out into the
    forest and sat down by the side of the cool fountain; and when she was bored she
    took a golden ball, and threw it up on high and caught it; and this ball was her
    favorite plaything.
"###;
