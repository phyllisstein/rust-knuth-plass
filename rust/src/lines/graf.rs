use crate::lines::nodes::*;
use std::collections::HashMap;


#[derive(Debug, Copy, Clone)]
pub struct Breakpoint {
    active: bool,
    position: usize,
    // total_demerits: u32,
    total_shrinkability: u32,
    total_stretchability: u32,
    total_width: u32,
}

#[derive(Debug, Clone)]
pub struct Graf {
    boxes: HashMap<usize, BoxNode>,
    breakpoints: HashMap<usize, Breakpoint>,
    glue: HashMap<usize, GlueNode>,
    plain_text: String,
}

impl Graf {
    const TARGET_LINE_LENGTH: usize = 60;

    pub fn new(plain_text: String) -> Graf {
        Graf {
            plain_text,
            boxes: HashMap::new(),
            glue: HashMap::new(),
            breakpoints: HashMap::new(),
        }
    }

    fn parse_nodes(&mut self) {
        for (position, grapheme) in self.plain_text.chars().enumerate() {
            if let Some(node) = BoxNode::from_char(grapheme, position) {
                self.boxes.insert(position, node);
            }

            if let Some(node) = GlueNode::from_char(grapheme, position) {
                self.glue.insert(position, node);
            }
        }
    }

    fn compute_breakpoints(&mut self) {
        let mut active_breakpoint = Breakpoint {
            active: true,
            position: 0,
            // total_demerits: 0,
            total_shrinkability: 0,
            total_stretchability: 0,
            total_width: 0,
        };

        for (&position, &glue) in self.glue.iter() {
            let mut new_breakpoint = active_breakpoint;

            // new_breakpoint.total_demerits += glue.total_demerits;
            new_breakpoint.total_shrinkability += glue.shrinkability;
            new_breakpoint.total_stretchability += glue.stretchability;
            new_breakpoint.total_width += glue.width;

            if new_breakpoint.total_width > Self::TARGET_LINE_LENGTH as u32 {
                new_breakpoint.active = false;
                self.breakpoints.insert(position, new_breakpoint);
                active_breakpoint = Breakpoint {
                    active: true,
                    position,
                    // total_demerits: 0,
                    total_shrinkability: 0,
                    total_stretchability: 0,
                    total_width: 0,
                };
            }
        }
    }

    pub fn get_hyphens(&mut self) -> String {
        let mut hyphens = String::from(&self.plain_text);

        self.parse_nodes();

        for (&position, breakpoint) in self.breakpoints.iter() {
            let new_position = breakpoint.position + (position * 5); // 5 is the length of `&shy;`
            hyphens.insert_str(new_position, "&shy;");
        }

        hyphens
    }
}
