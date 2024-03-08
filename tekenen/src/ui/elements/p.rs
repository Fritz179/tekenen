use std::{borrow::Borrow, cell::{Ref}};

use super::{DomElement, LayoutBox, LayoutNode, PaintElement, Stylable, TextFragment};
use crate::{colors, math::{IndefRange, Vec2}, platform::Event, shapes::rect::Rect, tekenen::Font, ui::style::{LayoutContext, Style}, Draw, Tekenen, Wrapper};

#[derive(Debug)]
enum Component {
    Fragment(TextFragment),
    Element(Box<dyn DomElement>)
}

#[derive(Debug)]
pub struct InnerP {
    pub style: Style,

    text: String,

    components: Vec<Component>
}

pub type P = Wrapper<InnerP>;

impl P {
    pub fn new(text: &str) -> Box<Self> {
        let mut style = Style::default();

        // TODO: should be 1rem
        style.margin.set_2(16.into(), 0.into());

        let p = Wrapper::wrap(InnerP {
            style,
            components: vec![],
            text: text.to_string()
        });

        let clone = p.clone() as Box<dyn DomElement>;

        p.borrow_mut().components.push(Component::Fragment(TextFragment::new(text, clone)));

        p
    }

    pub fn new_fn(text: &str, fun: impl FnOnce(&mut InnerP)) -> Box<Self> {
        let mut style = Style::default();

        // TODO: should be 1rem
        style.margin.set_2(16.into(), 0.into());

        let mut inner = InnerP {
            style,
            components: vec![],
            text: text.to_string()
        };

        fun(&mut inner);

        let p = Wrapper::wrap(inner);

        let clone = p.clone() as Box<dyn DomElement>;

        p.borrow_mut().components.push(Component::Fragment(TextFragment::new(text, clone)));

        p
    }
}

impl Stylable for P {
    fn get_style(&self) -> Ref<'_, Style> {
        Ref::map(self.0.as_ref().borrow(), |borrow| &borrow.style)
    }
}

impl DomElement for P {
    fn event(&mut self, event: Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn get_dom_children(&self) -> &Vec<Box<dyn DomElement>> {
        todo!()
    }

    fn get_layout_box(&self) -> LayoutNode {
        LayoutNode::new(self.clone())
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
    fn get_height_from_width(&self, width: i32, context: &LayoutContext) -> i32 {
        todo!()
    }

    fn get_width_from_height(&self, height: i32, context: &LayoutContext) -> i32 {
        todo!()
    }

    fn get_inner_min_max_content(&self, context: &LayoutContext) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_min_max_content(&self, context: LayoutContext) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_painter(&self, content_box: Rect, context: &LayoutContext) -> Box<dyn PaintElement> {
        self.clone()
    }

    fn is_inline(&self) -> bool {
        false
    }
}

impl PaintElement for P {
    fn draw(&self, target: &mut Tekenen, context: &LayoutContext, space: Vec2) {
        target.text_vec(&self.borrow().text, Vec2::zero(), Font::new(16, colors::BLUE));
    }
}