use super::{Element};
use crate::{Draw, Tekenen};

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: &str) -> Box<Self> {
        Box::new(Self {
            text: text.to_owned(),
        })
    }
}

impl Element for Text {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn draw(&mut self, target: &mut Tekenen) {
        // self.tv.draw_text(&self.text, 0, 0);
    }
}