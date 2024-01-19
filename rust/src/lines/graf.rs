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
        let nodes = Graf::parse_nodes(&plain_text);

        Graf {
            plain_text,
            nodes,
            feasible_breakpoints: vec![],
            active_breakpoints: vec![],
        }
    }

    fn parse_nodes(plain_text: &str) -> Vec<Node> {
        let mut nodes = vec![];

        for grapheme in UnicodeSegmentation::graphemes(plain_text, true) {
            if !LETTER_WIDTHS.contains_key(grapheme) {
                continue;
            }

            let node = match LETTER_WIDTHS[grapheme] {
                Node::Box { width } => Node::Box { width },
                Node::Glue {
                    width,
                    stretchability,
                    shrinkability,
                } => Node::Glue {
                    width,
                    stretchability,
                    shrinkability,
                },
                Node::Penalty {
                    width,
                    penalty,
                    flagged,
                } => Node::Penalty {
                    width,
                    penalty,
                    flagged,
                },
            };

            nodes.push(node);

            let glue = match grapheme {
                " " => WORD_GLUE,
                "," => COMMA_GLUE,
                ";" => SEMICOLON_GLUE,
                "." => PERIOD_GLUE,
                _ => continue,
            };

            nodes.push(glue);
        }

        nodes
    }

    pub fn to_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
}
