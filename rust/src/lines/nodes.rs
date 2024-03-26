use std::collections::HashMap;

// 'a' through 'z' are respectively (9, 10, 8, 10, 8, 6, 9, 10, 5, 6, 10, 5,
// 15,10, 9, 10, 10, 7, 7, 7, 10, 9, 13, 10, 10, 8) units wide and the
// characters 'C', 'I', and '-' have respective widths of 13, 6, and 6 units.
// Commas, semicolons, periods, and apostrophes occupy 5 units each. Glue has
// specifications (w, y, z) = (6, 3, 2) between words, except that it is (6, 4,
// 2) after a comma, (6, 4, 1) after a semicolon, and (8, 6, 1) after a period.
// A penalty of 50 has been assessed for every line that ends with a hyphen.
//      ---Donald E. Knuth, "Breaking Paragraphs Into Lines"
lazy_static! {
    static ref CHARACTER_BOX_WIDTHS: HashMap<char, u32> = HashMap::from([
        ('a', 9),
        ('A', 9),
        ('b', 10),
        ('B', 10),
        ('c', 8),
        ('C', 13),
        ('d', 10),
        ('D', 10),
        ('e', 8),
        ('E', 8),
        ('f', 6),
        ('F', 6),
        ('g', 9),
        ('G', 9),
        ('h', 10),
        ('H', 10),
        ('i', 5),
        ('I', 6),
        ('j', 6),
        ('J', 6),
        ('k', 10),
        ('K', 10),
        ('l', 5),
        ('L', 5),
        ('m', 15),
        ('M', 15),
        ('n', 10),
        ('N', 10),
        ('o', 9),
        ('O', 9),
        ('p', 10),
        ('P', 10),
        ('q', 10),
        ('Q', 10),
        ('r', 7),
        ('R', 7),
        ('s', 7),
        ('S', 7),
        ('t', 7),
        ('T', 7),
        ('u', 10),
        ('U', 10),
        ('v', 9),
        ('V', 9),
        ('w', 13),
        ('W', 13),
        ('x', 10),
        ('X', 10),
        ('y', 10),
        ('Y', 10),
        ('z', 8),
        ('Z', 8),
        ('-', 6),
        (',', 5),
        (';', 5),
        ('.', 5),
        (' ', 5),
    ]);

    // Tuple: (width, stretchability, shrinkability)
    static ref CHARACTER_GLUE_WIDTHS: HashMap<char, (u32, u32, u32)> = HashMap::from([
        (' ', (6, 3, 2)),
        (',', (6, 4, 2)),
        (';', (6, 4, 1)),
        ('.', (8, 6, 1)),
    ]);


}


pub trait Node {
    fn width(&self) -> u32;
}

#[derive(Debug)]
pub struct BoxNode {
    pub width: u32,
    pub grapheme: char,
}

impl Node for BoxNode {
    fn width(&self) -> u32 {
        self.width
    }
}

impl BoxNode {
    pub fn from_char(grapheme: char) -> BoxNode {
        match CHARACTER_BOX_WIDTHS.get(&grapheme) {
            Some(&width) => BoxNode { grapheme, width },
            _ => BoxNode { grapheme, width: 0 },
        }
    }

    pub fn from_values(width: u32, grapheme: char) -> BoxNode {
        BoxNode {
            width,
            grapheme,
        }
    }
}


#[derive(Debug)]
pub struct GlueNode {
    pub width: u32,
    pub stretchability: u32,
    pub shrinkability: u32,
    pub grapheme: char,
}

impl Node for GlueNode {
    fn width(&self) -> u32 {
        self.width
    }
}

impl GlueNode {
    pub fn from_char(grapheme: char) -> GlueNode {
        match CHARACTER_GLUE_WIDTHS.get(&grapheme) {
            Some(&(width, stretchability, shrinkability)) => GlueNode { width, stretchability, shrinkability, grapheme },
            _ => GlueNode { width: 0, stretchability: 0, shrinkability: 0, grapheme },
        }
    }

    pub fn from_values(width: u32, stretchability: u32, shrinkability: u32, grapheme: char) -> GlueNode {
        GlueNode {
            width,
            stretchability,
            shrinkability,
            grapheme,
        }
    }
}
