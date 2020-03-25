use rustbox::{Color, RustBox, Style, OutputMode};

pub struct View {
    rustbox: RustBox
}

impl View {
    pub fn new() -> Self {
        let mut rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}",e)
        };
        rustbox.set_output_mode(OutputMode::EightBit);
        View {
            rustbox
        }
    }

    pub fn get_box(&self) -> &RustBox {
        &self.rustbox
    }

    pub fn show(&self, text: Vec<Vec<Text>>) {
        self.rustbox.clear();
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut style = rustbox::RB_NORMAL;
        let mut fg = Color::White;
        let mut bg = Color::Black;
        for r in &text {
            for t in r {
                match t {
                    Text::Char(c) => {
                        self.rustbox.print_char(x,y,style,fg,bg,*c);
                        x += 1;
                    },
                    Text::FG(c) => {
                        fg = *c;
                    },
                    Text::BG(c) => {
                        bg = *c;
                    },
                    Text::Style(s) => {
                        style = *s;
                    },
                    Text::Skip(n) => {
                        x += n
                    }
                }
            }
            y += 1;
            x = 0;
        }
        self.rustbox.present();
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rustbox.width(), self.rustbox.height())
    }
}

#[derive(Clone,Copy,Debug)]
pub enum Text {
    Char(char),
    FG(Color),
    BG(Color),
    Style(Style),
    Skip(usize)
}