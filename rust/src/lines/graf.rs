use crate::lines::nodes::*;

#[derive(Debug, Copy, Clone)]
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
    boxes: Vec<BoxNode>,
    breakpoints: Vec<Breakpoint>,
    glue: Vec<GlueNode>,
    plain_text: String,
}

impl Graf {
    const TARGET_LINE_LENGTH: usize = 60;

    pub fn new(plain_text: String) -> Graf {
        Graf {
            plain_text,
            boxes: vec![],
            glue: vec![],
            breakpoints: vec![],
        }
    }

    fn parse_nodes(&mut self) {
        self.breakpoints.clear();
        self.breakpoints.push(Breakpoint {
            active: true,
            position: 0,
            total_demerits: 0,
            total_shrinkability: 0,
            total_stretchability: 0,
            total_width: 0,
        });

        for (position, grapheme) in self.plain_text.chars().into_iter().enumerate() {
            if let Some(node) = BoxNode::from_char(grapheme, position) {
                self.boxes.push(node);
            }

            if let Some(node) = GlueNode::from_char(grapheme, position) {
                self.glue.push(node);
            }
        }

        println!("{:?}", self.boxes);
        println!("{:?}", self.glue);
    }

    pub fn get_hyphens(&mut self) -> String {
        let mut hyphens = String::from(&self.plain_text);

        self.parse_nodes();

        for (position, breakpoint) in self.breakpoints.iter().enumerate() {
            let new_position = breakpoint.position + (position * 5); // 5 is the length of `&shy;`
            hyphens.insert_str(new_position, "&shy;");
        }

        hyphens
    }
}
