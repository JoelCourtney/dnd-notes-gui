mod header;
mod plain_text;
mod blank;
mod text_input;

use super::view::{Text,Text::*};
use rustbox::{RustBox, Color, Style};
use header::Header;
use plain_text::PlainText;
use blank::Blank;
use text_input::TextInput;

pub struct Controller<'a> {
    rustbox: &'a RustBox,
    columns: Vec<Column>,
    focus: Option<(usize, usize)>
}

impl<'a> Controller<'a> {
    pub fn new(rustbox: &'a RustBox) -> Self {
        let column = Column {elems:vec![
            Box::new(Header::new("DND Note Taker Tool".to_string())),
            Box::new(Blank::new(3)),
            Box::new(PlainText::new("Enter the name of a .json file:".to_string())),
            Box::new(TextInput::new())]};

        let mut result = Self {
            rustbox,
            columns: vec![column],
            focus: Some((0,0))
        };
        result.place_cursor();
        result
    }

    pub fn dump_text(&self, width: usize, height: usize) -> Vec<Vec<Text>> {
        let col_width: usize = {
            let hold = width / self.columns.len() - 3;
            if hold > 50 {
                50
            } else {
                hold
            }
        };

        let mut column_texts = Vec::new();
        for c in &self.columns {
            let mut text = Vec::new();
            for e in &c.elems {
                text.append(&mut e.dump_text(col_width, false));
            }
            column_texts.push(text);
        }
        let mut result = Vec::new();
        for r in 0..height {
            let mut row_text = Vec::new();
            for c in &mut column_texts {
                row_text.push(Char(' '));
                match c.get_mut(r) {
                    Some(v) => row_text.append(v),
                    None => row_text.push(Skip(col_width))
                }
                row_text.append(&mut vec![FG(Color::White), BG(Color::Black), Char(' '), Char('│')]);
            }
            row_text.pop();
            result.push(row_text);
        }
        result
    }

    fn place_cursor(&mut self) {
        let (mut c, mut e) = match self.focus {
            None => return,
            Some((c,e)) => (c,e)
        };
        loop {
            let col = &self.columns[c].elems;
            if e >= col.len() {
                self.focus = None;
                return;
            }
            if col.get(e).unwrap().can_have_focus() {
                self.focus = Some((c,e));
                return;
            }
            e += 1;
        }
    }

    pub fn process_input(&mut self, k: char) {
        self.rustbox.suspend(||panic!("help"));
        match self.focus {
            Some((c,e)) => self.columns[c].elems[e].process_input(k),
            None => {}
        }
    }
}

struct Column {
    elems: Vec<Box<dyn Element>>
}

trait Element {
    fn dump_text(&self, width: usize, active: bool) -> Vec<Vec<Text>>;

    fn string_to_text(&self, s: &str) -> Vec<Text> {
        let mut v = Vec::new();
        for c in s.chars() {
            v.push(Char(c));
        }
        v
    }
    
    fn can_have_focus(&self) -> bool {
        false
    }

    fn process_input(&mut self, _c: char) {
        panic!("asdf")
    }
}