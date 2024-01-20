use crate::lines::nodes::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Breakpoint {
    position: usize,
    line_number: i16,
    total_width: u32,
    total_stretchability: f32,
    total_shrinkability: f32,
    total_demerits: f32,
    previous_breakpoint: Option<Box<Breakpoint>>,
}

#[derive(Debug, Clone)]
pub struct Graf {
    plain_text: String,
    nodes: Vec<Node>,
    feasible_breakpoints: Vec<Breakpoint>,
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

    pub fn parse_nodes(&self, text: &String) -> (Vec<Node>, Vec<Breakpoint>) {
        let mut nodes = vec![];
        let mut breakpoints = vec![];

        for (position, grapheme) in text.graphemes(true).enumerate() {
            if let Some(node) = LETTER_BOXES.get(grapheme) {
                nodes.push(node.clone());
            } else if let Some(node) = PUNCTUATION_GLUE.get(grapheme) {
                if let Some(Node::Box { .. }) = nodes.last() {
                    let breakpoint = self.calculate_breakpoint(&nodes, position);
                    breakpoints.push(breakpoint);
                }

                nodes.push(node.clone());
            } else {
                nodes.push(Node::Null {
                    grapheme: grapheme.to_string(),
                });
            }
        }

        (nodes, breakpoints)
    }

    fn calculate_breakpoint(&self, nodes: &[Node], position: usize) -> Breakpoint {
        // FIXME: This should be `active_breakpoints` instead of `feasible_breakpoints`.
        let previous_breakpoint = self.feasible_breakpoints.last().unwrap_or(&Breakpoint {
            position: 0,
            line_number: 0,
            total_width: 0,
            total_stretchability: 0.0,
            total_shrinkability: 0.0,
            total_demerits: 0.0,
            previous_breakpoint: None,
        });

        let mut next_breakpoint = Breakpoint {
            position,
            // previous_breakpoint: Some(Box::new(previous_breakpoint.clone())),
            ..previous_breakpoint.clone()
        };

        let new_nodes = &nodes[previous_breakpoint.position..position];

        for node in new_nodes.iter() {
            let &width = match node {
                Node::Box { width } => width,
                Node::Glue { width, .. } => width,
                Node::Penalty { width, .. } => width,
                _ => &0,
            };

            next_breakpoint.total_width += width;

            if let Node::Glue {
                stretchability,
                shrinkability,
                ..
            } = node
            {
                next_breakpoint.total_stretchability += stretchability;
                next_breakpoint.total_shrinkability += shrinkability;
            }
        }

        next_breakpoint
    }

    pub fn parse(&mut self) -> &mut Graf {
        let (nodes, breakpoints) = self.parse_nodes(&self.plain_text);

        self.nodes = nodes;
        self.feasible_breakpoints = breakpoints;

        self
    }

    pub fn to_breakpoints(&self) -> &Vec<Breakpoint> {
        &self.feasible_breakpoints
    }
}
