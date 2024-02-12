use crate::lines::nodes::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Breakpoint {
    active: bool,
    position: usize,
    total_demerits: u32,
    total_shrinkability: u32,
    total_stretchability: u32,
    total_width: u32,
}

#[derive(Debug, Clone)]
pub struct Graf {
    plain_text: String,
    nodes: Vec<Node>,
    breakpoints: Vec<Breakpoint>,
}

impl Graf {
    const TARGET_LINE_LENGTH: usize = 60;

    pub fn new(plain_text: String) -> Graf {
        Graf {
            plain_text,
            nodes: vec![],
            breakpoints: vec![],
        }
    }

    fn parse_nodes(&mut self) {
        self.nodes.clear();

        self.breakpoints.clear();
        self.breakpoints.push(Breakpoint {
                active: true,
                position: 0,
                total_demerits: 0,
                total_shrinkability: 0,
                total_stretchability: 0,
                total_width: 0,
            }
        );

        for (position, grapheme) in self.plain_text.graphemes(true).enumerate() {
            if let Some(&node) = LETTER_BOXES.get(grapheme) {
                self.nodes.push(node);
            } else if let Some(&node) = PUNCTUATION_GLUE.get(grapheme) {
                self.nodes.push(node);

                if let Node::Box { .. } = self.nodes[position - 1] {
                    let breakpoint = self.calculate_breakpoint(position);
                    self.breakpoints.push(breakpoint);
                }
            }
        }
    }

    fn calculate_breakpoint(&self, position: usize) -> Breakpoint {
        let previous_breakpoint = self.breakpoints.last().unwrap();

        let mut next_breakpoint = Breakpoint {
            position,
            active: false,
            ..previous_breakpoint.clone()
        };

        let new_nodes = &self.nodes[previous_breakpoint.position..(position - 1)];

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
        };
        println!("{:?}", &next_breakpoint);

        next_breakpoint
    }

    // Last active breakpoint to current breakpoint.
    fn calculate_adjustment_ratio(&self) {}

    pub fn get_hyphens(&mut self) -> String {
        let mut hyphens = String::from(&self.plain_text);

        self.parse_nodes();

        for (position, breakpoint) in self.breakpoints.iter().enumerate() {
            let new_position = breakpoint.position + (position * 5); // 5 is the length of `&shy;`
            hyphens.insert_str(new_position, "&shy;");
        }

        hyphens
    }

    pub fn get_feasible_breakpoints(&mut self) -> Vec<Breakpoint> {
        self.parse_nodes();
        self.breakpoints.clone()
    }

    pub fn get_plain_text(&mut self) -> &String {
        &self.plain_text
    }
}
