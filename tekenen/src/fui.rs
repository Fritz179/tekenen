use std::{cell::Cell, fmt::{Debug, Display}, rc::Rc};

use div::{Div, InnerElement};

use crate::{platform::Event, shapes::rect::Rect, SurfaceView};

pub mod div;
pub mod text;

pub trait Element: Debug {
    fn event(&self, event: &Event);

    fn get_invalidation(&self) -> Invalidation;

    fn get_width(&self) -> i32;
    fn get_height(&self, width: i32) -> i32;

    fn draw(&self, ctx: &SurfaceView);
}

#[derive(Debug, Clone, Copy)]
pub enum Invalidation {
    Layout,
    Draw,
    None
}

impl Invalidation {
    fn needs_relayout(&self) -> bool {
        matches!(self, Self::Layout)
    }

    fn needs_redraw(&self) -> bool {
        matches!(self, Self::Layout | Self::Draw)
    }

    fn relayout(&mut self) {
        *self = Self::Layout;
    }

    fn relayout_if(&mut self, codition: bool) {
        if codition {
            self.relayout();
        }
    }

    fn redraw(&mut self) {
        if !self.needs_relayout() {
            *self = Self::Draw;
        }
    }

    fn redraw_if(&mut self, condition: bool) {
        if condition {
            self.redraw();
        }
    }

    fn merge(&mut self, other: Self) {
        if other.needs_relayout() {
            self.relayout();
        } else if other.needs_redraw() {
            self.redraw();
        }
    }

    fn merge_and_clear(&mut self, other: &Cell<Self>) {
        self.merge(other.get());
        other.set(Self::None);
    }
}

#[derive(Debug)]
pub struct FUI {
    element: InnerElement
}

impl FUI {
    pub fn new(fui: Rc<dyn Element>) -> Self {
        Self {
            element: InnerElement::new(fui)
        }
    }
}

impl FUI {
    pub fn render(&self, ctx: &SurfaceView) {        

        let invalidation = self.element.get_invalidation();

        if invalidation.needs_relayout() {
            println!("Relayout");

            let available_width = ctx.width();
            let requested_width = self.element.get_width();
            
            let width = available_width.min(requested_width);
            let height = self.element.get_height(width);

            self.element.clip(Rect::new(0, 0, width, height));
        }

        // TODO: Should not always have to redraw
        // if invalidation.needs_redraw() {
            let ctx = ctx.tee();
            self.element.draw(&ctx);
        // }
    }

    pub fn event(&self, event: &Event) {
        
    }
}