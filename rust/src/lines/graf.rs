use crate::lines::nodes::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct Breakpoint {
    position: i16,
    line_number: i16,
    total_width: u32,
    total_stretchability: f32,
    total_shrinkability: f32,
    total_demerits: f32,
    previous_breakpoint: Option<Box<Breakpoint>>,
}

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
            if !LETTER_WIDTHS.contains_key(grapheme) {
                continue;
            }

            let node = match LETTER_WIDTHS[grapheme] {
                Node::Box { width } => Node::Box {
                    width,
                },
                Node::Glue { width, stretchability, shrinkability } => Node::Glue {
                    width,
                    stretchability,
                    shrinkability,
                },
                Node::Penalty { width, penalty, flagged } => Node::Penalty {
                    width,
                    penalty,
                    flagged,
                },
            };

            self.nodes.push(node);

            let glue = match grapheme {
                " " => WORD_GLUE,
                "," => COMMA_GLUE,
                ";" => SEMICOLON_GLUE,
                "." => PERIOD_GLUE,
                _ => continue,
            };

            self.nodes.push(glue);
        }
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn to_vec(&self) -> Vec<&Node> {
        self.nodes.iter().collect()
    }
}
