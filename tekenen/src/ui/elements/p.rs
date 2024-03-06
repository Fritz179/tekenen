use std::{cell::RefCell, rc::Rc};

use super::{DomElement, TextFragment};
use crate::{colors, math::{IndefRange, Vec2}, tekenen::Font, ui::style::{LayoutContext, Style}, Draw, Tekenen};

#[derive(Debug)]
enum Component {
    Fragment(TextFragment),
    Element(Rc<RefCell<dyn DomElement>>)
}

#[derive(Debug)]
pub struct P {
    pub style: Style,

    components: Vec<Component>
}

impl P {
    pub fn new(text: &str) -> Rc<RefCell<Self>> {
        let mut bb = Style::default();

        // TODO: should be 1rem
        bb.margin.set_2(16.into(), 0.into());

        let mut this = Rc::new(RefCell::new(P {
            components: vec![],
            style: bb
        }));

        let clone = Rc::clone(&this) as Rc<RefCell<dyn DomElement>>;

        this.borrow_mut().components.push(Component::Fragment(TextFragment::new(text, clone)));

        this
    }

    pub fn new_fn(text: &str, fun: impl FnOnce(&mut Self)) -> Rc<RefCell<Self>> {
        let mut bb = Style::default();

        // TODO: should be 1rem
        bb.margin.set_2(16.into(), 0.into());

        let mut p = P {
            components: vec![],
            style: bb
        };

        fun(&mut p);

        let this = Rc::new(RefCell::new(p));

        let clone = Rc::clone(&this) as Rc<RefCell<dyn DomElement>>;

        this.borrow_mut().components.push(Component::Fragment(TextFragment::new(text, clone)));

        this
    }
}

impl DomElement for P {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn get_dom_children(&self) -> &Vec<Rc<RefCell<dyn DomElement>>> {
        todo!()
    }

    fn get_layout_box(&self, target: Rc<RefCell<dyn DomElement>>) -> super::LayoutNode<dyn super::LayoutBox> {
        todo!()
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

    fn get_style(&self) -> &Style {
        &self.style
    }
}