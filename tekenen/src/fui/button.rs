use std::{cell::{Cell, RefCell}, rc::Rc};

use crate::{colors, platform::Event, printer::Print, DrawableSurface};

use super::{Element, Invalidation};

pub struct Button {
    width: i32,
    height: i32,
    pressed: Cell<bool>,
    dirty: Cell<Invalidation>,
    on_click: RefCell<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new(on_click: impl FnMut() + 'static) -> Rc<Self> {
        Rc::new(Self {
            width: 100,
            height: 50,
            pressed: Cell::new(false),
            dirty: Cell::new(Invalidation::Layout),
            on_click: RefCell::new(Box::new(on_click))
        })
    }
}

impl Print for Button {
    fn print(&self, printer: &mut crate::printer::Printer) -> std::fmt::Result {
        printer.println(&"<Button>")?;
        printer.indent(2);
        printer.print_previous()?;
        printer.property("pressed", &self.pressed)?;
        printer.debug().property("dirty", &self.dirty)
    }
}

impl Element for Button {
    fn event(&self, event: crate::platform::Event) {
        if let Event::MouseDown { x, y, .. } = event {
            self.pressed.set(true);
            self.dirty.set(Invalidation::Draw);
            (self.on_click.borrow_mut())();
        }

        if self.pressed.get() {
            if let Event::MouseUp { x, y, .. } = event {
                self.pressed.set(false);
                self.dirty.set(Invalidation::Draw);
            }
        }
    }

    fn get_invalidation(&self) -> super::Invalidation {
        self.dirty.replace(Invalidation::None)
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self, width: i32) -> i32 {
        self.height
    }

    fn draw(&self, ctx: &crate::SurfaceView) {
        let color = if self.pressed.get() { colors::MAGENTA } else { colors::RED };
        ctx.background(color)
    }
}