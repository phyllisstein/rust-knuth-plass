use crate::lines::nodes::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Breakpoint {
    position: usize,
    line_number: i16,
    total_width: u32,
    total_stretchability: u32,
    total_shrinkability: u32,
    total_demerits: u32,
}

#[derive(Debug, Clone)]
pub struct Graf {
    plain_text: String,
    nodes: Vec<Node>,
    all_breakpoints: Vec<Breakpoint>,
    active_breakpoints: Vec<Breakpoint>,
}

impl Graf {
    const TARGET_LINE_LENGTH: i32 = 60;

    pub fn new(plain_text: String) -> Graf {
        Graf {
            plain_text,
            nodes: vec![],
            all_breakpoints: vec![],
            active_breakpoints: vec![],
        }
    }

    fn parse_nodes(&mut self) {
        let mut nodes: Vec<Node> = vec![];
        let mut breakpoints: Vec<Breakpoint> = vec![];

        for (position, grapheme) in self.plain_text.graphemes(true).enumerate() {
            if let Some(&node) = LETTER_BOXES.get(grapheme) {
                nodes.push(node);
            } else if let Some(&node) = PUNCTUATION_GLUE.get(grapheme) {
                if let Some(Node::Box { .. }) = nodes.last() {
                    let breakpoint = self.calculate_breakpoint(&nodes, position);
                    breakpoints.push(breakpoint);
                }

                nodes.push(node);
            }
        }

        self.nodes.append(&mut nodes);
        self.all_breakpoints.append(&mut breakpoints);
    }

    fn calculate_breakpoint(&self, nodes: &[Node], position: usize) -> Breakpoint {
        let previous_breakpoint = match self.all_breakpoints.last() {
            Some(&Breakpoint {
                position,
                line_number,
                total_width,
                total_stretchability,
                total_shrinkability,
                total_demerits,
            }) => Breakpoint {
                position,
                line_number,
                total_width,
                total_stretchability,
                total_shrinkability,
                total_demerits,
            },
            _ => Breakpoint {
                position: 0,
                line_number: 0,
                total_width: 0,
                total_stretchability: 0,
                total_shrinkability: 0,
                total_demerits: 0,
            },
        };

        let mut next_breakpoint = Breakpoint {
            position,
            ..previous_breakpoint
        };

        let new_nodes = &nodes[previous_breakpoint.position..position];

        for node in new_nodes.iter() {
            let width = match node {
                Node::Box { width } => width,
                Node::Glue { width, .. } => width,
                Node::Penalty { width, .. } => width,
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
        println!("{:?}", next_breakpoint);

        next_breakpoint
    }

    fn calculate_adjustment_ratio(&self) {}

    fn find_active_breakpoints(&mut self) {}

    fn compute_adjustment_ratio(&mut self, lhs: Breakpoint, rhs: Breakpoint) {
        let mut adjustment_ratio = 0;
        let intervening_nodes = self.nodes.iter().take_while(|node| {
            node.position >= lhs.position && node.position <= rhs.position;
        });
    }

    pub fn get_hyphens(&mut self) -> String {
        let mut hyphens = String::from(&self.plain_text);

        self.parse_nodes();

        for (position, breakpoint) in self.all_breakpoints.iter().enumerate() {
            let new_position = breakpoint.position + (position * 5);
            hyphens.insert_str(new_position, "&shy;");
        }

        hyphens
    }

    pub fn get_feasible_breakpoints(&mut self) -> &Vec<Breakpoint> {
        self.parse_nodes();
        &self.all_breakpoints
    }

    pub fn get_active_breakpoints(&mut self) -> &Vec<Breakpoint> {
        self.parse_nodes();
        &self.active_breakpoints
    }

    pub fn get_plain_text(&mut self) -> &String {
        &self.plain_text
    }
}
