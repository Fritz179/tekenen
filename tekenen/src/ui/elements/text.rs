use std::{cell::RefCell, rc::Rc};

use super::{Element, SpaceContraint};
use crate::{math::Vec2, tekenen::Font, Draw, Tekenen};

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub height: i32,
}

impl Text {
    pub fn new(text: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Text {
            text: text.to_owned(),
            height: 8
        }))
        
    }
}

impl Element for Text {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn draw(&self, target: &mut Tekenen, available_space: Vec2) -> Vec2 {
        target.text(&self.text, 0, 0, Font::new(self.height))
    }

    fn get_layout(&self) -> SpaceContraint {
        SpaceContraint::Area(self.text.len() as i32 * self.height * self.height, (self.height, self.text.len() as i32 * self.height))
    }
}