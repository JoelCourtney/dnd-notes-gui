use super::super::view::{Text,Text::*};
use super::Element;
use rustbox::{Color};

pub struct PlainText {
    text: String
}

impl PlainText {
    pub fn new(s: String) -> Self {
        Self {
            text: s
        }
    }
}

impl Element for PlainText {
    fn dump_text(&self, width: usize, _active: bool) -> Vec<Vec<Text>> {
        let mut v = Vec::new();
        if self.text.len() <= width {
            let skip = width - self.text.len();
            v.append(&mut self.string_to_text(&self.text));
            v.push(Skip(skip));
        }
        vec![v]
    }
}