use std::{rc::Rc, cell::RefCell};

use super::{Element, UIBuilder};
use crate::{Draw, TransforView};

pub struct Text {
    text: String,
    tv: Box<dyn Draw>
}

impl Text {
    pub fn new(text: &str, tv: Box<dyn Draw>) -> Box<Self> {
        Box::new(Self {
            text: text.to_owned(),
            tv
        })
    }
}

impl Element for Text {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        self.tv.draw_text(&self.text, 0, 0);
    }
}

impl UIBuilder {
    pub fn text(mut self, text: &str) -> Self {
        self.children.push(Text::new(text, Box::new(TransforView::new(0, 0, 0, 0, self.tv.clone()))));
        self
    }
}