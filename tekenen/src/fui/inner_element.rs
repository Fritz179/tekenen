use std::{cell::Cell, rc::Rc};

use crate::{platform::Event, printer::{Print, Printer}, shapes::rect::Rect, SurfaceView};

use super::{Element, Invalidation};

#[derive(Debug)]
pub struct InnerElement {
    element: Rc<dyn Element>,
    clip: Cell<Option<Rect>>,
    width: Cell<Option<i32>>,
    height: Cell<Option<(i32, i32)>>,
}

impl InnerElement {
    pub fn new(element: Rc<dyn Element>) -> Self {
        Self {
            element,
            clip: Cell::new(None),
            width: Cell::new(None),
            height: Cell::new(None),
        }
    }
}

impl Element for InnerElement {
    fn event(&self, event: &Event) {
        self.element.event(event);
    }

    fn get_invalidation(&self) -> Invalidation {
        let validation = self.element.get_invalidation();

        if validation.needs_relayout() {
            self.width.set(None);
            self.height.set(None);
        }

        validation
    }

    fn get_width(&self) -> i32 {
        if let Some(width) = self.width.get() {
            return width;
        }

        let width = self.element.get_width();
        self.width.set(Some(width));
        width
    }

    fn get_height(&self, width: i32) -> i32 {
        if let Some((previous_width, height)) = self.height.get() {
            if previous_width == width {
                return height;
            }
        }

        let height = self.element.get_height(width);
        self.height.set(Some((width, height)));
        height
    }

    fn draw(&self, tekenen: &SurfaceView) {
        tekenen.clip(self.clip.get().expect("Element has no bounding box!"));
        self.element.draw(tekenen);
    }
}

impl InnerElement {
    pub fn clip(&self, clip: Rect) {
        self.clip.set(Some(clip));
    }
}

impl Print for InnerElement {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.set_previous(self.clip.get().map(|e| format!("{e}")).unwrap_or("No Bounding box".to_owned()));
        printer.println(&*self.element)
    }
}