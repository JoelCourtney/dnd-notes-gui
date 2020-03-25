// use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

mod dnd;
mod view;
mod ui;

use dnd::Campaign;
use view::View;
use ui::Controller;

use rustbox::{Key};

fn main() {
    // let camp = Campaign::read("test.json");
    // camp.write("test.json");
    let view = View::new();
    let mut controller = Controller::new(view.get_box());
    loop {
        let (w,h) = view.size();
        view.show(controller.dump_text(w,h));
        match view.get_box().poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char(c) => controller.process_input(c),
                    Key::Esc => break,
                    _ => {}
                }
            },
            Err(e) => panic!("{}",e),
            _ => {}
        }
    }
}