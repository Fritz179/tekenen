use std::{cell::{RefCell, RefMut}, ops::Deref, rc::Rc};

use super::{Element, ElementBox, SpaceContraint};
use crate::{math::Vec2, tekenen::Font, Draw, Tekenen};

#[derive(Debug)]
pub struct Text {
    pub text: String,
    pub height: i32,
}

pub struct TextElement {
    element: Rc<RefCell<Text>>
}

impl TextElement {
    pub fn new(text: &str) -> Self {
        TextElement {
            element: Rc::new(RefCell::new(Text {
                text: text.to_owned(),
                height: 8
            }))
        }
    }

    pub fn rc_clone(&self) -> Self {
        Self {
            element: Rc::clone(&self.element)
        }
    }
}

impl From<TextElement> for Rc<RefCell<dyn Element>> {
    fn from(text: TextElement) -> Rc<RefCell<dyn Element>> {
        text.element
    }
}

impl ElementBox for TextElement {
    type InnerElement = Text;

    fn get(&self) -> RefMut<'_, Self::InnerElement> {
        self.element.deref().borrow_mut()
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