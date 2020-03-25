use super::super::view::{Text,Text::*};
use super::Element;
use rustbox::{Color};

pub struct Header {
    text: String
}

impl Header {
    pub fn new(s: String) -> Self {
        Self {
            text: s
        }
    }
}

impl Element for Header {
    fn dump_text(&self, width: usize, _active: bool) -> Vec<Vec<Text>> {
        let mut v = vec![FG(Color::Blue)];
        if self.text.len() <= width {
            let skip = width - self.text.len();
            let left_margin: usize = skip / 2;
            let right_margin = skip - left_margin;
            v.push(Skip(left_margin));
            v.append(&mut self.string_to_text(&self.text));
            v.push(Skip(right_margin));
        }
        vec![v]
    }
}