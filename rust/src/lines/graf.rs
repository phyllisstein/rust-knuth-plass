use crate::lines::nodes::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Breakpoint {
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
    feasible_breakpoints: Vec<Breakpoint>,
    active_breakpoints: Vec<Breakpoint>,
}

impl Graf {
    pub fn new(plain_text: String) -> Graf {
        let (nodes, feasible_breakpoints) = Graf::parse_nodes(&plain_text);

        Graf {
            plain_text,
            nodes,
            feasible_breakpoints,
            active_breakpoints: vec![],
        }
    }

    fn parse_nodes(plain_text: &str) -> (Vec<Node>, Vec<Breakpoint>) {
        let mut nodes = vec![];
        let mut breakpoints = vec![];

        for grapheme in UnicodeSegmentation::graphemes(plain_text, true) {
            if let Some(node) = LETTER_BOXES.get(&grapheme) {
                nodes.push(node.clone());
            } else if let Some(node) = PUNCTUATION_GLUE.get(&grapheme) {
                if let Some(Node::Box { .. }) = nodes.last() {
                    let breakpoint = Breakpoint {
                        position: nodes.len() as i16,
                        line_number: 0,
                        total_width: 0,
                        total_stretchability: 0.0,
                        total_shrinkability: 0.0,
                        total_demerits: 0.0,
                        previous_breakpoint: None,
                    };
                    breakpoints.push(breakpoint);
                }

                nodes.push(node.clone());
            } else {
                nodes.push(Node::Null {});
            }
        }

        (nodes, breakpoints)
    }

    pub fn to_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn to_breakpoints(&self) -> &Vec<Breakpoint> {
        &self.feasible_breakpoints
    }
}
