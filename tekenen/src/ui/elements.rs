pub mod div;
use std::{cell::{Ref}, fmt::Debug};

pub use div::Div;

// pub mod slider;
// pub use slider::Slider;

pub mod textFragment;
pub use textFragment::TextFragment;

pub mod p;
pub use p::P;

use crate::{math::{IndefRange, Vec2}, platform::Event, shapes::rect::Rect, Draw, Tekenen};


use super::style::{LayoutContext, Style};

pub trait Stylable: Debug {
    fn get_style(&self) -> Ref<'_, Style>;
}

// Every HTML element has to implement this Trait
pub trait DomElement: Stylable {
    // React to event
    fn event(&mut self, event: Event);

    // Called once before layout and draw
    fn update(&mut self);

    // Implement for default behavior
    fn get_dom_children(&self) -> &Vec<Box<dyn DomElement>>;

    // Implement to get a Layout/Box Tree
    fn get_layout_box(&self) -> LayoutNode;
}

// Every entry in the Layout/Box tree has to implemnet this 
pub trait LayoutBox: Stylable {
    fn get_min_max_content(&self, context: LayoutContext) -> Vec2<IndefRange>;

    // Get space constraints for layouting
    fn get_inner_min_max_content(&self, context: &LayoutContext) -> Vec2<IndefRange>;

    // Used for layouting
    fn get_width_from_height(&self, height: i32, context: &LayoutContext) -> i32;
    fn get_height_from_width(&self, width: i32, context: &LayoutContext) -> i32;

    fn get_painter(&self, content_box: Rect, context: &LayoutContext) -> Box<dyn PaintElement>;

    fn is_inline(&self) -> bool;
}

// A block-level box is a box that participates in a block formatting context
pub trait BlockLayoutBox: LayoutBox {

}

// An inline-level box is a box that participates in an inline formatting context
pub trait InlineLayoutBox: LayoutBox {
    fn split_into_line(&self, formatter: InlineFormattingContext);
}

// This is what the tree is made of
// Each sub_context initiates a new context type
// Each children can either be a Block or Inline level box
// Formatting Context are not limited to this two (Flex, Grid, Table...)
#[derive(Debug)]
pub struct LayoutNode {
    node: Box<dyn LayoutBox>,
    children: Vec<LayoutNode>
}

impl LayoutNode {
    pub fn new(node: Box<dyn LayoutBox>) -> Self {
        Self {
            node,
            children: vec![]
        }
    }
}


// Every Formatting Context must be able to differentiate between block- and inline-level boxes
impl LayoutNode {
    fn add_node(&mut self, node: LayoutNode) {
        self.children.push(node)
    }
}

pub struct BlockFormattingContext {
    
}

impl Default for BlockFormattingContext {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockFormattingContext {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn get_painter(&self, node: &LayoutNode, space: Rect) -> PainterTree {
        let context = LayoutContext {
            containing_block: space
        };

        // Walk through all children
        let mut inline = None;
        let mut children = vec![];

        for child in node.children.iter() {
            if child.node.is_inline() {
                let inline = inline.get_or_insert(InlineFormattingContext::new());

                todo!();
                // inline.get_painter()
            } else {
                let child = self.get_painter(child, space);
                children.push(child);
            }
        }

        let height: i32 = children.iter().map(|child| child.margin_box.size.y).sum();

        let content_box = Rect::new_vec(space.position, Vec2::new(space.size.x, height));

        let element = node.node.get_painter(content_box, &context);

        PainterTree {
            margin_box: content_box,
            border_box: content_box,
            padding_box: content_box,
            content_box,
            children,
            element,
            context
        }
    }
}

pub struct InlineFormattingContext {

}

impl Default for InlineFormattingContext {
    fn default() -> Self {
        Self::new()
    }
}

impl InlineFormattingContext {
    pub fn new() -> Self {
        Self {

        }
    }
}

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
    pub parent: Box<dyn DomElement>,
    pub value: String
}

// impl BlockLayoutBox for AnonymousTextBox {
//     fn get_painter(&self, content_box: Rect, context: LayoutContext) -> PainterTree {
//         todo!()
    
//     }
// }

pub trait PaintElement: Stylable {
    fn draw(&self, target: &mut Tekenen, context: &LayoutContext, space: Vec2);
}

#[derive(Debug)]
pub struct PainterTree {
    pub margin_box: Rect,
    pub border_box: Rect,
    pub padding_box: Rect,
    pub content_box: Rect,
    pub element: Box<dyn PaintElement>,
    pub context: LayoutContext,
    pub children: Vec<PainterTree>
}

impl PainterTree {
    pub fn paint(&self, target: &mut Tekenen) {
        // let element = self.element.borrow();
        // let style = element.get_style();

        // let bg_color = style.background_color.solve(&self.context);

        // if bg_color[3] > 0 {
        //     target.set_translation_vec(self.border_box.position);
        //     target.rect_vec(Vec2::zero(), self.border_box.size, bg_color);
        // }

        target.set_translation_vec(self.content_box.position);

        self.element.draw(target, &self.context, self.content_box.size);

        for element in self.children.iter() {
            element.paint(target);
        }
    }
}