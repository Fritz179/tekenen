use std::{cell::{Cell, RefCell}, rc::Rc};

use crate::{colors, math::Vec2, platform::Event, printer::Print, DrawableSurface};

use super::{Element, Invalidation};

const RADIUS: i32 = 10;

pub struct Slider {
    width: Cell<i32>,
    height: Cell<i32>,
    value: Cell<f32>,
    moving: Cell<bool>,
    dirty: Cell<Invalidation>,
    callback: RefCell<Box<dyn FnMut(f32)>>
}

impl Slider {
    pub fn new(callback: impl FnMut(f32) + 'static) -> Rc<Self> {
        Rc::new(Self {
            width: Cell::new(100),
            height: Cell::new(RADIUS * 2),
            value: Cell::new(0.5),
            moving: Cell::new(false),
            dirty: Cell::new(Invalidation::Layout),
            callback: RefCell::new(Box::new(callback))
        })
    }
}

impl Print for Slider {
    fn print(&self, printer: &mut crate::printer::Printer) -> std::fmt::Result {
        printer.println(&"<Slider>")?;
        printer.indent(2);
        printer.property("value", &self.value)?;
        printer.property("moving", &self.moving)?;
        printer.debug().property("width", &self.width)?;
        printer.debug().property("height", &self.height)?;
        printer.debug().property("dirty", &self.dirty)
    }
}

impl Slider {
    fn slider_position(&self) -> Vec2 {
        let y = self.height.get() / 2;
        let x = (self.width.get() as f32 * self.value.get()) as i32;
        Vec2::new(x, y)
    }
}

impl Element for Slider {
    fn event(&self, event: Event) {
        match event {
            Event::MouseDown { x, y } => {
                let pos = self.slider_position();

                if (x - pos.x) * (x - pos.x) + (y - pos.y) * (y - pos.y) < RADIUS * RADIUS {
                    self.moving.set(true);
                }
            },
            Event::MouseMove { x, .. } => {
                if self.moving.get() {
                    let x = x.max(0).min(self.width.get());
                    let value = x as f32 / self.width.get() as f32;

                    self.value.set(value);
                    self.dirty.set(Invalidation::Draw);

                    (self.callback.borrow_mut())(value);
                }
            },
            Event::MouseUp { .. } => {
                self.moving.set(false);
            },
            _ => {}
        }
    }

    fn get_width(&self) -> i32 {
        self.width.get()
    }

    fn get_height(&self, width: i32) -> i32 {
        self.height.get()
    }

    fn get_invalidation(&self) -> Invalidation {
        self.dirty.replace(Invalidation::None)
    }

    fn draw(&self, ctx: &crate::SurfaceView) {
        let pos = self.slider_position();

        // Slider
        ctx.fill_color(colors::WHITE);
        ctx.rect(0, RADIUS / 2, self.width.get(), RADIUS);

        // Dot
        ctx.fill_color(colors::GREEN);
        ctx.circle(pos.x, pos.y, RADIUS);
    }
}