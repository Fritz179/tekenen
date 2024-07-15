use std::{cell::RefCell, rc::Rc};

use crate::{math::{IndefRange, Vec2, Zero}, shapes::rect::Rect, html::style::{CSSDisplayShorthand, FormattingInfo}, DrawableSurface, SurfaceView};

use super::{BlockLayoutBox, DomElement, InlineFormattingContext, LayoutBox, PaintElement, Stylable, Style};


/// A div is a flexbox
/// A div with a single element is a flexbox with a single element
#[derive(Debug)]
pub struct Div {
    pub style: RefCell<Style>,
    children: RefCell<Vec<Rc<dyn DomElement>>,>
}

impl Div {
    pub fn new(children: Vec<Rc<dyn DomElement>>) -> Rc<Self> {
        let mut style = Style::default();

        style.display = CSSDisplayShorthand::Block.into();

        Rc::new(Self {
            style: RefCell::new(style),
            children: RefCell::new(children)
        })
    }

    pub fn new_fn(children: Vec<Rc<dyn DomElement>>, fun: impl FnOnce(&mut Div)) -> Rc<Self> {
        let mut style = Style::default();

        style.display = CSSDisplayShorthand::Block.into();

        let mut div = Self {
            style: RefCell::new(style),
            children: RefCell::new(children)
        };

        fun(&mut div);

        Rc::new(div)
    }
}

impl Stylable for Div {
    fn get_style(&self) -> &RefCell<Style> {
        &self.style
    }

    fn get_name(&self) -> String {
        "div".to_owned()
    }
}

impl DomElement for Div {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn get_layout_box(self: Rc<Self>) -> Rc<dyn LayoutBox> {
        self
    }

    fn get_dom_children(&self) -> Option<&RefCell<Vec<Rc<dyn DomElement>>>> {
        Some(&self.children)
    }
}

impl LayoutBox for Div {
    fn get_height_from_width(&self, width: i32, context: &FormattingInfo) -> i32 {
        todo!()
    }

    fn get_width_from_height(&self, height: i32, context: &FormattingInfo) -> i32 {
        todo!()
    }

    fn get_inner_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_painter(self: Rc<Self>, content_box: Rect, context: &FormattingInfo) -> Rc<dyn PaintElement> {
        self
    }

    fn is_inline(&self) -> bool {
        false
    }

    fn go_inline_yourself(&self, inline: &InlineFormattingContext, info: &FormattingInfo) -> Vec<(Rc<super::LineBox>, Rc<dyn LayoutBox>)> {
        todo!()
    }
}

impl BlockLayoutBox for Div {

}

impl PaintElement for Div {
    fn draw(&self, target: &mut SurfaceView, context: &FormattingInfo, space: Vec2) {
        let color = self.get_style().borrow().background_color.solve(context);

        if color[3] > 0 {
            target.rect_vec(Vec2::zero(), space, color)
        }
    }
}