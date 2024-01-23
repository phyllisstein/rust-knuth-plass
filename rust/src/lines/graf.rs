use crate::lines::nodes::*;
use std::cell::RefCell;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Breakpoint {
    position: usize,
    line_number: i32,
    total_width: i32,
    total_stretchability: i32,
    total_shrinkability: i32,
    total_demerits: i32,
}

#[derive(Debug, Clone)]
pub struct Graf {
    plain_text: String,
    nodes: RefCell<Vec<Node>>,
    feasible_breakpoints: RefCell<Vec<Breakpoint>>,
    active_breakpoints: RefCell<Vec<Breakpoint>>,
}

impl Graf {
    const MAX_LINE_WIDTH: i32 = 60;

    pub fn new(plain_text: String) -> Graf {
        Graf {
            plain_text,
            nodes: RefCell::new(vec![]),
            feasible_breakpoints: RefCell::new(vec![]),
            active_breakpoints: RefCell::new(vec![]),
        }
    }

    fn parse_nodes(&self) {
        let mut nodes: Vec<Node> = vec![];
        let mut breakpoints: Vec<Breakpoint> = vec![];

        for (position, grapheme) in self.plain_text.graphemes(true).enumerate() {
            if let Some(&node) = LETTER_BOXES.get(grapheme) {
                nodes.push(node);
            } else if let Some(&node) = PUNCTUATION_GLUE.get(grapheme)
                && let Some(Node::Box { .. }) = nodes.last()
            {
                let breakpoint = self.calculate_breakpoint(&nodes, position);
                breakpoints.push(breakpoint);
                nodes.push(node);
            }
        }

        self.nodes.replace(nodes);
        self.feasible_breakpoints.replace(breakpoints);
    }

    fn calculate_breakpoint(&self, nodes: &[Node], position: usize) -> Breakpoint {
        let feasible_breakpoints = self.feasible_breakpoints.borrow();
        let &previous_breakpoint = feasible_breakpoints.last().unwrap_or(&Breakpoint {
            position: 0,
            line_number: 0,
            total_width: 0,
            total_stretchability: 0,
            total_shrinkability: 0,
            total_demerits: 0,
        });

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

        next_breakpoint
    }

    fn find_active_breakpoints(&self) {
        let mut active_breakpoints: Vec<Breakpoint> = vec![];
        let iter = self.feasible_breakpoints.borrow();

        for bps in self.feasible_breakpoints.borrow().iter() {
            let mut adjust_ratio = 0.0;
            let &previous_breakpoint =
                self.active_breakpoints
                    .borrow()
                    .last()
                    .unwrap_or(&Breakpoint {
                        position: 0,
                        line_number: 0,
                        total_width: 0,
                        total_stretchability: 0,
                        total_shrinkability: 0,
                        total_demerits: 0,
                    });

            let next_breakpoint = bps;

            let line_width = next_breakpoint.total_width - previous_breakpoint.total_width;
            let line_stretchability =
                next_breakpoint.total_stretchability - previous_breakpoint.total_stretchability;
            let line_shrinkability =
                next_breakpoint.total_shrinkability - previous_breakpoint.total_shrinkability;

            if line_width > Self::MAX_LINE_WIDTH {
                adjust_ratio = ((line_width - Self::MAX_LINE_WIDTH)
                    / next_breakpoint.total_shrinkability) as f64;
            } else if line_width < Self::MAX_LINE_WIDTH {
                adjust_ratio = ((Self::MAX_LINE_WIDTH - line_width) / next_breakpoint.total_stretchability) as f64;
            }

            if line_stretchability > 0.0 {
                adjust_ratio += adjust_ratio * adjust_ratio / line_stretchability;
            } else if line_shrinkability > 0.0 {
                adjust_ratio += adjust_ratio * adjust_ratio / line_shrinkability;
            }

            let mut active_breakpoint = Breakpoint {
                position: next_breakpoint.position,
                line_number: previous_breakpoint.line_number + 1,
                total_width: next_breakpoint.total_width,
                total_stretchability: next_breakpoint.total_stretchability,
                total_shrinkability: next_breakpoint.total_shrinkability,
                total_demerits: previous_breakpoint.total_demerits + adjust_ratio,
            };

            if let Some(Node::Penalty { penalty, .. }) =
                self.nodes.borrow().get(active_breakpoint.position)
            {
                active_breakpoint.total_demerits += *penalty as f32;
            }

            active_breakpoints.push(active_breakpoint);
        }
    }

    pub fn get_hyphens(&self) -> String {
        let mut hyphens = String::from(&self.plain_text);

        for (position, breakpoint) in self.feasible_breakpoints.borrow().iter().enumerate() {
            let new_position = breakpoint.position + (position * 5);
            hyphens.insert_str(new_position, "&shy;");
        }

        hyphens
    }

    pub fn parse(&self) -> &Graf {
        self.parse_nodes();
        self
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.nodes.borrow().clone()
    }

    pub fn get_feasible_breakpoints(&self) -> Vec<Breakpoint> {
        self.feasible_breakpoints.borrow().clone()
    }

    pub fn get_active_breakpoints(&self) -> Vec<Breakpoint> {
        self.active_breakpoints.borrow().clone()
    }

    pub fn get_plain_text(&self) -> String {
        self.plain_text.clone()
    }
}
