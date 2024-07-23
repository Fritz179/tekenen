use std::{cell::{Cell, RefCell}, rc::Rc};

use crate::{platform::Event, printer::{Print, Printer}, shapes::rect::Rect, SurfaceView};

use super::{inner_element::InnerElement, Element, Invalidation};

#[derive(Debug)]
pub struct Div {
    children: RefCell<Vec<InnerElement>>,
    invalidation: Cell<Invalidation>,
}

impl Div {
    pub fn new(children: Vec<Rc<dyn Element>>) -> Rc<Self> {
        Rc::new(Self {
            children: RefCell::new(children.into_iter().map(|child| InnerElement::new(child)).collect()),
            invalidation: Cell::new(Invalidation::Layout),
        })
    }

    pub fn add_child(self: Rc<Self>, child: Rc<dyn Element>) -> Rc<Self> {
        self.children.borrow_mut().push(InnerElement::new(child));
        self
    }
}

impl Print for Div {
    fn print(&self, printer: &mut Printer) -> std::fmt::Result {
        printer.println(&"<Div>")?;
        printer.indent(2);
        printer.print_previous()?;
        printer.println(&"children:")?;
        printer.indent(6);

        for child in self.children.borrow_mut().iter() {
            printer.println(child)?;
        }

        Ok(())
    }
}

impl Element for Div {
    fn event(&self, event: &Event) {
        for child in self.children.borrow_mut().iter() {
            child.event(event);
        }
    }

    fn get_invalidation(&self) -> Invalidation {
        let mut validation = self.invalidation.replace(Invalidation::None);

        for child in self.children.borrow_mut().iter() {
            let child_validation = child.get_invalidation();
            validation.merge(child_validation);
        }

        validation
    }

    fn get_width(&self) -> i32 {
        let mut width = 0;

        for child in self.children.borrow_mut().iter() {
            width += child.get_width();
        }

        width
    }

    fn get_height(&self, width: i32) -> i32 {
        let mut height = 0;

        for child in self.children.borrow_mut().iter() {
            let child_height = child.get_height(width);
            child.clip(Rect::new(0, height, width, child_height));
            height += child_height;
        }

        height
    }

    fn draw(&self, tekenen: &SurfaceView) {
        for child in self.children.borrow_mut().iter() {
            child.draw(tekenen);
        }
    }
}