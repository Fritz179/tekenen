pub mod div;
use std::{alloc::Layout, cell::RefCell, fmt::Debug, rc::Rc};

pub use div::Div;

// pub mod slider;
// pub use slider::Slider;

pub mod textFragment;
pub use textFragment::TextFragment;

pub mod p;
pub use p::P;

use crate::{math::{IndefRange, Vec2}, platform::Event, shapes::rect::Rect, Draw, Tekenen};


use super::style::{LayoutContext, Style};

pub trait DomElement: std::fmt::Debug + std::any::Any {
    // React to event
    fn event(&mut self, event: Event);

    // Called once before layout and draw
    fn update(&mut self);

    // Implement for default behavior
    fn get_dom_children(&self) -> &Vec<Rc<RefCell<dyn DomElement>>>;

    // Implement to get a Layout/Box Tree
    fn get_layout_box(&self, target: Rc<RefCell<dyn DomElement>>) -> LayoutNode<dyn LayoutBox>;

    // Get the bounding box
    fn get_style(&self) -> &Style;

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
}

pub trait LayoutBox: Debug {
    fn get_min_max_content(&self, context: LayoutContext) -> Vec2<IndefRange>;

    // Get space constraints for layouting
    fn get_inner_min_max_content(&self, context: &LayoutContext) -> Vec2<IndefRange>;

    // Used for layouting
    fn get_width_from_height(&self, height: i32, context: &LayoutContext) -> i32;
    fn get_height_from_width(&self, width: i32, context: &LayoutContext) -> i32;
}

// Every Node of the Layout / Box Tree implements this trait
pub trait BlockLayoutBox: LayoutBox {
    fn get_painter(&self, content_box: Rect, context: LayoutContext) -> PainterTree;

}

// An inline-level box is a box that participates in an inline formatting context
pub trait InlineLayoutBox: BlockLayoutBox {
    fn split_into_line(&self, formatter: InlineFormattingContext);
}

#[derive(Debug)]
enum LayoutNodeVec {
    Block(Vec<Rc<RefCell<dyn BlockLayoutBox>>>),
    Inline(Vec<Rc<RefCell<dyn InlineLayoutBox>>>),
}

#[derive(Debug)]
// Principal Box
pub struct LayoutNode<T: LayoutBox + ?Sized> {
    node: Rc<RefCell<T>>,
    children: Vec<LayoutNodeVec>
}

impl<T: LayoutBox + ?Sized> LayoutNode<T> {
    pub fn new(node: Rc<RefCell<T>>) -> Self {
        Self {
            node,
            children: Vec::new()
        }
    }
}

impl LayoutNode<dyn LayoutBox> {
    pub fn get_painter(&self, content_box: Rect, context: LayoutContext) -> PainterTree {
        todo!()
    }
}

impl<T: LayoutBox + ?Sized> LayoutBox for LayoutNode<T> {
    fn get_min_max_content(&self, context: LayoutContext) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_inner_min_max_content(&self, context: &LayoutContext) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_width_from_height(&self, height: i32, context: &LayoutContext) -> i32 {
        todo!()
    }

    fn get_height_from_width(&self, width: i32, context: &LayoutContext) -> i32 {
        todo!()
    }
}

// Block Box with Block Children
impl BlockLayoutBox for LayoutNode<dyn BlockLayoutBox> {
    fn get_painter(&self, content_box: Rect, context: LayoutContext) -> PainterTree {
        todo!()
    }
}

// Atomic Inline-Level Box
impl InlineLayoutBox for LayoutNode<dyn BlockLayoutBox> {
    fn split_into_line(&self, formatter: InlineFormattingContext) {
        todo!()
    }
}

// Needed beacuse InlineLayoutBox is a supertrait of BlockLayoutBox
impl BlockLayoutBox for LayoutNode<dyn InlineLayoutBox> {
    fn get_painter(&self, content_box: Rect, context: LayoutContext) -> PainterTree {
        todo!()
    }
}

// Inline Box with Inline Children
impl InlineLayoutBox for LayoutNode<dyn InlineLayoutBox> {
    fn split_into_line(&self, formatter: InlineFormattingContext) {
        todo!()
    }
}


// Every Formatting Context must be able to differentiate between block- and inline-level boxes
impl<T: BlockLayoutBox + ?Sized> LayoutNode<T> {
    fn add_block_box(&mut self, box_: Rc<RefCell<dyn BlockLayoutBox>>) {
        match self.children.last_mut() {
            // Empty or Inline
            None | Some(LayoutNodeVec::Inline(_)) => {
                self.children.push(LayoutNodeVec::Block(vec![box_]));
            },
            Some(LayoutNodeVec::Block(vec)) => {
                vec.push(box_);
            },
        }
    }

    fn add_inline_box(&mut self, box_: Rc<RefCell<dyn InlineLayoutBox>>) {
        match self.children.last_mut() {
            // Empty or Inline
            None | Some(LayoutNodeVec::Block(_)) => {
                self.children.push(LayoutNodeVec::Inline(vec![box_]));
            },
            Some(LayoutNodeVec::Inline(vec)) => {
                vec.push(box_);
            },
        }
    }
}

struct InlineFormattingContext {

}

// The root of the layout tree
// Lay out the children in a vertical stack
// #[derive(Debug, Default)]
// pub struct BlockFormattingContext {
//     boxes_to_block: Vec<Rc<RefCell<BlockLevelBox>>>,
//     last_inline_context: Option<Rc<RefCell<BlockLevelBox>>>,
// }

// impl FormattingContext for BlockFormattingContext {


//     fn get_next_inline_available_width(&self) -> (i32, bool) {
//         todo!()
//     }
// }


// impl BlockFormattingContext {
//     fn new() -> Self {
//         Self::default()
//     }
// }

// The line box is a horizontal stack of inline-level boxes
#[derive(Debug)]
struct LineBox {
    boxes: Vec<Box<dyn BlockLayoutBox>>,
    width: i32,
    height: i32
}


// Lay out the children in a vertical stack of line boxes
// #[derive(Debug, Default)]
// struct InlineFormattingContext {
//     boxes_to_inline: Vec<Rc<RefCell<dyn BlockLayoutBox>>>,
//     line_boxes: Vec<LineBox>,
//     floats: Vec<BlockFormattingContext>
// }

// impl FormattingContext for InlineFormattingContext {

//     fn get_next_inline_available_width(&self) -> (i32, bool) {
//         todo!()
//     }
// }

// impl InlineFormattingContext {
//     fn new() -> Self {
//         Self::default()
//     }

//     fn get_next_line_box(&self) -> (LineBox, bool) {
//         todo!()
//     }
// }

// fn get_painter(&self, content_box: Rect, context: LayoutContext) -> PainterTree {
//     let target = self.element.borrow();
//     let style = target.get_style();

//     let padding_box = content_box + style.get_computed_padding(&context);
//     let border_box = padding_box + style.get_computed_border(&context);
//     let margin_box = border_box + style.get_computed_margin(&context);

//     let child_context = LayoutContext {
//         containing_block: content_box
//     };

//     let children = target.get_children_painters(&child_context, content_box.size);

//     PainterTree {
//         margin_box,
//         border_box,
//         padding_box,
//         content_box,
//         element: Rc::clone(&self.element),
//         context,
//         children
//     }
// }


#[derive(Debug)]
struct AnonymousTextBox {
    pub parent: Rc<RefCell<dyn DomElement>>,
    pub value: String
}

// impl BlockLayoutBox for AnonymousTextBox {
//     fn get_painter(&self, content_box: Rect, context: LayoutContext) -> PainterTree {
//         todo!()
    
//     }
// }

trait PaintElement: Debug {
    fn draw(&self, target: &mut Tekenen, context: &LayoutContext, space: Vec2);
    fn get_style(&self) -> &Style;
}

#[derive(Debug)]
pub struct PainterTree {
    pub margin_box: Rect,
    pub border_box: Rect,
    pub padding_box: Rect,
    pub content_box: Rect,
    pub element: Rc<RefCell<dyn PaintElement>>,
    pub context: LayoutContext,
    pub children: Vec<PainterTree>
}

impl PainterTree {
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