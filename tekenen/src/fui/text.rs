use std::{cell::{Cell, RefCell}, rc::Rc};

use crate::{platform::Event, DrawableSurface, SurfaceView};

use super::{Element, Invalidation};

#[derive(Debug)]
pub struct Text {
    text: RefCell<String>,
    dirty: Cell<Invalidation>,
}

impl Text {
    pub fn new(text: &str) -> Rc<Self> {
        Rc::new(Self {
            text: RefCell::new(text.to_string()),
            dirty: Cell::new(Invalidation::Layout),
        })
    }

    pub fn set_text(&self, text: String) {
        let mut old_text = self.text.borrow_mut();

        // Length chaged, relayout
        if old_text.len() != text.len() {
            *old_text = text;
            self.dirty.set(Invalidation::Layout);
            return
        }

        // Same length but text changed, redraw only
        if *old_text != text {
            *old_text = text;
            self.dirty.set(Invalidation::Draw);
        }
    }
}

impl Element for Text {
    fn event(&self, _event: &Event) {
        
    }

    fn get_invalidation(&self) -> Invalidation {
        self.dirty.replace(Invalidation::None)
    }

    fn get_width(&self) -> i32 {
        16 * self.text.borrow().len() as i32
    }

    fn get_height(&self, width: i32) -> i32 {
        (self.get_width() + width - 1) / width * 16
    }

    fn draw(&self, tekenen: &SurfaceView) {
        tekenen.text(&self.text.borrow(), 0, 0, 16);
    }
}