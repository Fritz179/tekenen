use std::{cell::{Cell, RefCell}, rc::Rc};

use crate::{platform::Event, shapes::rect::Rect, SurfaceView};

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

#[derive(Debug)]
pub struct Div {
    children: RefCell<Vec<InnerElement>>,
    invalidation: Cell<Invalidation>,
}

impl Div {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            children: RefCell::new(Vec::new()),
            invalidation: Cell::new(Invalidation::Layout),
        })
    }

    pub fn add_child(self: Rc<Self>, child: Rc<dyn Element>) -> Rc<Self> {
        self.children.borrow_mut().push(InnerElement::new(child));
        self
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