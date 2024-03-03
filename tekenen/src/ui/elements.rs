pub mod div;
use std::{cell::RefCell, rc::Rc};

pub use div::Div;

pub mod slider;
pub use slider::Slider;

pub mod text;
pub use text::Text;

use crate::{math::{IndefRange, Vec2}, platform::Event, shapes::rect::Rect, Draw, Tekenen};


use super::style::{Context, Style};

pub trait Element: std::fmt::Debug {
    // React to event
    fn event(&mut self, event: Event);

    // Called once before layout and draw
    fn update(&mut self);

    // Get space constraints for layouting
    fn get_inner_min_max_content(&self, context: &Context) -> Vec2<IndefRange>;

    // Used for layouting
    fn get_width_from_height(&self, height: i32, context: &Context) -> i32;
    fn get_height_from_width(&self, width: i32, context: &Context) -> i32;

    // Get children if any
    fn get_children_painters(&self, context: &Context, size: Vec2) -> Vec<Painter> {
        vec![]
    }

    // Draw onto target withing given space
    fn draw(&self, target: &mut Tekenen, context: &Context, space: Vec2);

    // Get the bounding box
    fn get_style(&self) -> &Style;
}

#[derive(Debug)]
pub struct Painter {
    pub margin_box: Rect,
    pub border_box: Rect,
    pub padding_box: Rect,
    pub content_box: Rect,
    pub element: Rc<RefCell<dyn Element>>,
    pub context: Context,
    pub children: Vec<Painter>
}

impl Painter {
    pub fn paint(&self, target: &mut Tekenen) {
        let element = self.element.borrow();
        let style = element.get_style();

        let bg_color = style.background_color.solve(&self.context);

        if bg_color[3] > 0 {
            target.set_translation_vec(self.border_box.position);
            target.rect_vec(Vec2::zero(), self.border_box.size, bg_color);
        }

        target.set_translation_vec(self.content_box.position);

        element.draw(target, &self.context, self.content_box.size);

        for element in self.children.iter() {
            element.paint(target);
        }
    }
}