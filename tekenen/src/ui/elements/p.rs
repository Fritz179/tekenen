use std::{borrow::Borrow, cell::{Ref, RefCell}, rc::Rc};

use super::{DomElement, LayoutBox, LayoutNode, PaintElement, Stylable, TextNode};
use crate::{colors, math::{IndefRange, Vec2}, platform::Event, shapes::rect::Rect, tekenen::Font, ui::style::{FormattingInfo, Style}, Draw, Tekenen};

#[derive(Debug)]
pub struct P {
    pub style: RefCell<Style>,

    children: RefCell<Vec<Rc<dyn DomElement>>>
}

impl P {
    pub fn new(text: &str) -> Rc<Self> {
        let mut style = Style::default();

        // TODO: should be 1rem
        style.margin.set_2(16.into(), 0.into());

        let p = Rc::new(Self {
            style: RefCell::new(style),
            children: RefCell::new(vec![]),
        });
        
        p.children.borrow_mut().push(TextNode::new(text));

        p
    }

    pub fn new_fn(text: &str, fun: impl FnOnce(&mut Self)) -> Rc<Self> {
        let mut style = Style::default();

        // TODO: should be 1rem
        style.margin.set_2(16.into(), 0.into());

        let mut inner = Self {
            style: RefCell::new(style),
            children: RefCell::new(vec![]),
        };

        fun(&mut inner);

        let p = Rc::new(inner);

        p.children.borrow_mut().push(TextNode::new(text));

        p
    }
}

impl Stylable for P {
    fn get_style(&self) -> &RefCell<Style> {
        &self.style
    }

    fn get_name(&self) -> String {
        "p".to_owned()
    }
}

impl DomElement for P {
    fn event(&mut self, event: Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn get_dom_children(&self) -> Option<&RefCell<Vec<Rc<dyn DomElement>>>> {
        None
    }

    fn get_layout_box(self: Rc<Self>) -> LayoutNode {
        let mut node = LayoutNode::new(Rc::clone(&self) as Rc<dyn LayoutBox>);

        for child in self.children.borrow().iter() {
            node.add_node(Rc::clone(child).get_layout_box())
        }

        node
    }

    // fn draw(&self, target: &mut Tekenen, context: &LayoutContext, space: Vec2) {
    //     // for (i, line) in self.split_text(space.x, context).iter().enumerate() {
    //     //     println!("{line}");
    //     //     target.text(&line, 0, i as i32 * self.height, Font::new(self.height as i32, colors::BLACK));
    //     // }
    // }

    // fn get_height_from_width(&self, width: i32, context: &LayoutContext) -> i32 {
    //     self.split_text(width, context).len() as i32 * 16
    // }

    // fn get_width_from_height(&self, height: i32, context: &LayoutContext) -> i32 {
    //     self.text.len() as i32 * 16
    // }

    // fn get_inner_min_max_content(&self, context: &LayoutContext) -> Vec2<IndefRange> {
    //     // TODO: Get the actual size of the text
    //     Vec2::new(IndefRange::new(16, self.text.len() as i32 * 16), IndefRange::new(16, 16))
    // }
}

impl LayoutBox for P {
    fn get_height_from_width(&self, width: i32, context: &FormattingInfo) -> i32 {
        16
    }

    fn get_width_from_height(&self, height: i32, context: &FormattingInfo) -> i32 {
        todo!()
    }

    fn get_inner_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_min_max_content(&self, context: FormattingInfo) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_painter(self: Rc<Self>, content_box: Rect, context: &FormattingInfo) -> Rc<dyn PaintElement> {
        self.clone()
    }

    fn is_inline(&self) -> bool {
        false
    }

    fn go_inline_yourself(&self, formatter: &mut super::InlineFormattingContext, context: &dyn super::FormattingContext, info: &FormattingInfo) 
            -> Vec<(Rc<super::LineBox>, Rc<dyn LayoutBox>)> {
        todo!()
    }
}

impl PaintElement for P {
    fn draw(&self, target: &mut Tekenen, context: &FormattingInfo, space: Vec2) {
        let color = self.get_style().borrow().background_color.solve(context);

        if color[3] > 0 {
            target.rect_vec(Vec2::zero(), space, color)
        }
    }
}