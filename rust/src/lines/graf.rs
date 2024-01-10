use crate::lines::constants::{Box, Glue, Penalty};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum Node {
    Box(Box),
    Glue(Glue),
    Penalty(Penalty),
}


#[derive(Debug)]
pub struct Graf {
    plain_text: String,
    nodes: Vec<Node>,
}

impl Graf {
    pub fn new(plain_text: String) -> Graf {
        Graf {
            plain_text,
            nodes: vec![],
        }
    }

    pub fn parse_nodes(&mut self) {
        for (_index, grapheme) in self.plain_text.graphemes(true).enumerate() {
            let node = match grapheme {
                " " => Node::Glue(Glue {
                    width: 3,
                    stretchability: 1.0,
                    shrinkability: 1.0,
                }),
                "-" => Node::Penalty(Penalty {
                    width: 0,
                    penalty: 100,
                    flagged: false,
                }),
                _ => Node::Box(Box { width: 1 }),
            };

            self.nodes.push(node);
        }
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn to_vec(&self) -> Vec<&Node> {
        self.nodes.iter().collect()
    }
}
