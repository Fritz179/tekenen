use std::{cell::{RefCell, RefMut}, ops::Deref, rc::Rc};

use super::{Element, ElementBox};
use crate::{tekenen::Font, Draw, Tekenen};

pub struct Text {
    pub text: String,
}

pub struct TextElement {
    pub element: Rc<RefCell<Text>>
}

impl TextElement {
    pub fn new(text: &str) -> Self {
        TextElement {
            element: Rc::new(RefCell::new(Text {
                text: text.to_owned(),
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

    fn draw(&mut self, target: &mut Tekenen) {
        target.text(&self.text, 0, 0, Font::new(8));
    }
}