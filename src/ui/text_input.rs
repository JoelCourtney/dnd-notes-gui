use super::super::view::{Text,Text::*};
use super::Element;
use rustbox::{Color};

pub struct TextInput {
    text: String
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            text: "hello ".to_string()
        }
    }
}

impl Element for TextInput {
    fn dump_text(&self, width: usize, _active: bool) -> Vec<Vec<Text>> {
        let mut top = Vec::new();
        top.append(&mut vec![FG(Color::Blue),Char('┌')]);
        for _ in 1..width-1 {
            top.push(Char('─'));
        }
        top.push(Char('┐'));
        let mut result = Vec::new();
        result.push(top);
        let mut text = self.text.clone();
        while text.len() != 0 {
            let mut v = vec![FG(Color::Blue),Char('│'),FG(Color::White)];
            if text.len() > width - 2 {
                v.append(&mut self.string_to_text(&text[0..width-2]));
                text = text[width-2..].to_string();
            } else {
                let skip = width - 2 - text.len();
                v.append(&mut self.string_to_text(&text));
                v.push(Skip(skip));
                text = "".to_string();
            }
            v.append(&mut vec![FG(Color::Blue),Char('│')]);
            result.push(v);
        }
        let mut bottom = Vec::new();
        bottom.append(&mut vec![FG(Color::Blue),Char('└')]);
        for i in 1..width-1 {
            bottom.push(Char('─'));
        }
        bottom.push(Char('┘'));
        result.push(bottom);
        result
    }

    fn can_have_focus(&self) -> bool {
        true
    }
}