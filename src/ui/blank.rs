use super::super::view::{Text,Text::*};
use super::Element;
use rustbox::{Color};

pub struct Blank {
    height: usize
}

impl Blank {
    pub fn new(n: usize) -> Self {
        Self {
            height: n
        }
    }
}

impl Element for Blank {
    fn dump_text(&self, width: usize, _active: bool) -> Vec<Vec<Text>> {
        let mut v = Vec::new();
        for i in 0..self.height {
            v.push(vec![Skip(width)]);
        }
        v
    }
}