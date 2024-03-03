use std::{cell::RefCell, rc::Rc};

use super::Element;
use crate::{math::{IndefRange, Vec2}, tekenen::Font, ui::style::{Context, Style}, Draw, Tekenen};

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub height: i32,
    pub style: Style
}

impl Text {
    pub fn new(text: &str) -> Rc<RefCell<Self>> {
        let mut bb = Style::default();

        // TODO: should be 1rem
        bb.margin.set_2(16.into(), 0.into());

        Rc::new(RefCell::new(Text {
            text: text.to_owned(),
            height: 8,
            style: bb
        }))
    }

    pub fn new_fn(text: &str, fun: impl FnOnce(&mut Self)) -> Rc<RefCell<Self>> {
        let mut bb = Style::default();

        // TODO: should be 1rem
        bb.margin.set_2(16.into(), 0.into());

        let mut text = Text {
            text: text.to_owned(),
            height: 8,
            style: bb
        };

        fun(&mut text);

        Rc::new(RefCell::new(text))
    }
}

impl Element for Text {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn draw(&self, target: &mut Tekenen, context: &Context, space: Vec2) {
        target.text(&self.text, 0, 0, Font::new(self.height));
    }

    fn get_height_from_width(&self, width: i32, context: &Context) -> i32 {
        16
    }

    fn get_width_from_height(&self, height: i32, context: &Context) -> i32 {
        self.text.len() as i32 * 16
    }

    fn get_inner_min_max_content(&self, context: &Context) -> Vec2<IndefRange> {
        // TODO: Get the actual size of the text
        Vec2::new(IndefRange::new(16, self.text.len() as i32 * 16), IndefRange::new(16, 16))
    }

    fn get_style(&self) -> &Style {
        &self.style
    }
}