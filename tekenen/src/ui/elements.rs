pub mod div;
use std::{cell::Ref, fmt::{Debug, Display}};

pub use div::Div;

// pub mod slider;
// pub use slider::Slider;

pub mod textFragment;
pub use textFragment::TextFragment;

pub mod p;
pub use p::P;

use crate::{math::{IndefRange, Vec2}, platform::Event, shapes::rect::Rect, Draw, Tekenen, Wrapper};


use super::style::{FormattingInfo, Style};

pub trait Stylable: Debug {
    fn get_style(&self) -> Ref<'_, Style>;
    fn get_name(&self) -> String;
}

// Every HTML element has to implement this Trait
pub trait DomElement: Stylable + Display {
    // React to event
    fn event(&mut self, event: Event);

    // Called once before layout and draw
    fn update(&mut self);

    // Implement for default behavior
    fn get_dom_children(&self) -> Option<Ref<'_, Vec<Box<dyn DomElement>>>>;

    // Implement to get a Layout/Box Tree
    fn get_layout_box(&self) -> LayoutNode;
}

impl<T> Display for Wrapper<T> where Wrapper<T>: DomElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);

        writeln!(f, "{:width$}{}", "", self.get_name())?;

        let width = width + 4;
        if let Some(children) = self.get_dom_children() {
            for child in children.iter() {
                write!(f, "{:width$}", child)?;
            }
        }

        Ok(())
    }
}

// Every entry in the Layout/Box tree has to implemnet this 
pub trait LayoutBox: Stylable {
    fn get_min_max_content(&self, context: FormattingInfo) -> Vec2<IndefRange>;

    // Get space constraints for layouting
    fn get_inner_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange>;

    // Used for layouting
    fn get_width_from_height(&self, height: i32, context: &FormattingInfo) -> i32;
    fn get_height_from_width(&self, width: i32, context: &FormattingInfo) -> i32;

    fn get_painter(&self, content_box: Rect, context: &FormattingInfo) -> Box<dyn PaintElement>;

    fn is_inline(&self) -> bool;

    fn create_formatting_context_if_needed(&self) -> Option<Box<dyn BlockFormattingContext>> {
        match self.get_style().display {
            super::style::CSSDisplay::Flex => Some(Box::new(FlexFormattingContext::new())),
            _ => None   
        }
    }

    fn go_inline_yourself(&self, formatter: &mut InlineFormattingContext, context: &dyn FormattingContext, info: &FormattingInfo) -> Vec<(Box<LineBox>, Box<dyn LayoutBox>)>;
}


// A block-level box is a box that participates in a block formatting context
pub trait BlockLayoutBox: LayoutBox {

}

// An inline-level box is a box that participates in an inline formatting context
pub trait InlineLayoutBox: LayoutBox {
    fn split_into_line(&self, formatter: InlineFormattingContext);
}

#[derive(Debug)]
enum ContextDecision {
    BlockContext,
    InlineContext,
    InlineElement,
    FlexContext,
}

// This is what the tree is made of
// Each children can either be a Block or Inline level box
// Formatting Context are not limited to this two (Flex, Grid, Table...)
#[derive(Debug)]
pub struct LayoutNode {
    element: Box<dyn LayoutBox>,
    children: Vec<LayoutNode>,
    context: Option<ContextDecision>
}

impl LayoutNode {
    pub fn new(node: Box<dyn LayoutBox>) -> Self {
        Self {
            element: node,
            children: vec![],
            context: None
        }
    }
}

impl LayoutNode {
    fn add_node(&mut self, node: LayoutNode) {
        self.children.push(node)
    }
}

pub trait FormattingContext {
    fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Box<LineBox>, bool);
    fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Box<LineBox>, bool);
}

pub trait BlockFormattingContext: FormattingContext {
    fn run(&self, node: &LayoutNode, info: &FormattingInfo) -> PainterTree;
}

pub struct BlockBlockFormattingContext {
    
}

impl BlockBlockFormattingContext {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl FormattingContext for BlockBlockFormattingContext {
    fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Box<LineBox>, bool) {
        todo!()
    }

    fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Box<LineBox>, bool) {
        todo!()
    }
}

impl BlockBlockFormattingContext {
    fn run_parent(&self, node: &LayoutNode, info: &FormattingInfo) -> PainterTree {
        // Apply CSS Definite sizes
        let mut available_size = info.containing_block;

        if let Some(width) = node.element.get_style().width.solve(info) {
            available_size.size.x = width;
        }

        if let Some(height) = node.element.get_style().height.solve(info) {
            available_size.size.y = height;
        }

        // Get inner size for children
        let bounding = node.element.get_style().get_total_computed_boudning(&info);
        let available_content_rect = info.containing_block - bounding.clone();

        // Walk through all children
        let mut inline = None;
        let mut children = vec![];

        let mut child_info = FormattingInfo {
            containing_block: available_content_rect
        };

        for child in node.children.iter() {
            if child.element.is_inline() {
                let inline = inline.get_or_insert(InlineFormattingContext::new());

                inline.add_inline(child, self, info)
            } else {
                if let Some(inline) = inline {
                    let mut child = inline.run( self, &child_info);
                    // child_info.containing_block.position.y += child.margin_box.size.y;
                    children.append(&mut child);
                }

                inline = None;

                let child = self.run(child, &child_info);
                child_info.containing_block.position.y += child.margin_box.size.y;
                children.push(child);
            }
        }

        if let Some(inline) = inline {
            let mut child = inline.run( self, &child_info);
            // child_info.containing_block.position.y += child.margin_box.size.y;
            children.append(&mut child);
        }

        let inner_height = child_info.containing_block.position.y - info.containing_block.position.y;
        let outer_height = inner_height + bounding.top + bounding.bottom;
        let outer_width = info.containing_block.size.x;

        let margin_box = Rect::new_vec(info.containing_block.position, Vec2::new(outer_width, outer_height));
        let content_box = margin_box - bounding;

        let element = node.element.get_painter(content_box, &info);

        PainterTree {
            margin_box: content_box,
            border_box: content_box,
            padding_box: content_box,
            content_box,
            children,
            element,
            context: info.clone()
        }
    }
}

impl BlockFormattingContext for BlockBlockFormattingContext {
    fn run(&self, node: &LayoutNode, info: &FormattingInfo) -> PainterTree {
        if let Some(context) = node.element.create_formatting_context_if_needed() {
            return context.run(node, info);
        }

        if !node.children.is_empty() {
            return self.run_parent(node, info)
        }

        // We are a leaf node!

        // get CSS restrictions
        let size_constraint = node.element.get_style().get_size_contraint(&info);

        // Get inner width
        let boudning = node.element.get_style().get_total_computed_boudning(&info);
        let outer_width = info.containing_block.size.x;
        let inner_width = outer_width - boudning.left - boudning.right;

        // Get inner height
        let inner_height = node.element.get_height_from_width(inner_width, &info);
        
        // Get outer dimensions
        let outer_height = inner_height + boudning.top + boudning.bottom;
        let margin_box = Rect::new_vec(info.containing_block.position, Vec2::new(outer_width, outer_height));
        let content_box = margin_box - boudning;

        let element = node.element.get_painter(content_box, &info);

        dbg!(content_box);

        PainterTree {
            margin_box: content_box,
            border_box: content_box,
            padding_box: content_box,
            content_box,
            children: vec![],
            element,
            context: info.clone()
        }
    }
}


pub struct InlineFormattingContext<'a> {
    pub lines: Vec<Box<LineBox>>,
    /// parent, all its children
    elements: Vec<(&'a LayoutNode, Vec<(Box<LineBox>, Box<dyn LayoutBox>)>)>,
}

impl<'a> InlineFormattingContext<'a> {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            elements: vec![]
        }
    }
}

impl<'a> FormattingContext for  InlineFormattingContext<'a> {
    fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Box<LineBox>, bool) {
        if self.lines.is_empty() {
            return self.get_new_line(context, info);
        }

        return (self.lines.last().unwrap().as_ref().clone(), false);
    }

    fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Box<LineBox>, bool) {
        let line = Wrapper::wrap(LineBoxInner {
            boxes: vec![],
            width: info.containing_block.size.x,
            height: 0
        });

        self.lines.push(line.clone());

        return (line, true);
    }
}

impl<'a> InlineFormattingContext<'a> {
    fn run(&self, context: &dyn FormattingContext, info: &FormattingInfo) -> Vec<PainterTree> {
        // We are closing the InlineFormattingContext

        // Calculate the height of each line
        for line in self.lines.iter() {
            let mut line = line.borrow_mut();
            let boxes = &line.boxes;

            assert!(boxes.len() == 1, "We only support one box per line");

            let box_ = &boxes[0];
            let height = box_.get_height_from_width(info.containing_block.size.x, info);
            line.height = height;
        }

        self.elements.iter().map(|(node, children)| {
            // TODO: Everything

            assert!(children.len() == 1, "We only support one child per line");
            let (line, piece) = &children[0];

            let containing_block = Rect::new_vec(
                info.containing_block.position, 
                Vec2::new(info.containing_block.size.x, line.borrow().height)
            );

            let piece = piece.get_painter(containing_block, info);
            let element = node.element.get_painter(containing_block, &info);

            PainterTree {
                margin_box: info.containing_block,
                border_box: info.containing_block,
                padding_box: info.containing_block,
                content_box: info.containing_block,
                element,
                context: info.clone(),
                children: vec![PainterTree {
                    margin_box: info.containing_block,
                    border_box: info.containing_block,
                    padding_box: info.containing_block,
                    content_box: info.containing_block,
                    element: piece,
                    context: info.clone(),
                    children: vec![]
                }]
            }
        }).collect()
    }

    fn add_inline_parent(&mut self, node: &LayoutNode, context: &dyn FormattingContext, info: &FormattingInfo) {
        todo!()
    }
    
    fn add_inline(&mut self, node: &'a LayoutNode, context: &dyn FormattingContext, info: &FormattingInfo) {
        if !node.children.is_empty() {
            return self.add_inline_parent(node, context, info)
        }

        let children = node.element.go_inline_yourself(self, context, info);
        self.elements.push((node, children));
    }
}

struct FlexFormattingContext {

}

impl FlexFormattingContext {
    fn new() -> Self {
        Self {

        }
    }
}

impl FormattingContext for FlexFormattingContext {
    fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Box<LineBox>, bool) {
        todo!()
    }

    fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Box<LineBox>, bool) {
        todo!()
    }
}

impl BlockFormattingContext for FlexFormattingContext {
    fn run(&self, node: &LayoutNode, info: &FormattingInfo) -> PainterTree {
        todo!()
    }
}

// The line box is a horizontal stack of inline-level boxes
#[derive(Debug)]
pub struct LineBoxInner {
    boxes: Vec<Box<dyn LayoutBox>>,
    width: i32,
    height: i32
}

pub type LineBox = Wrapper<LineBoxInner>;

impl LineBox {
    pub fn add(&self, element: Box<dyn LayoutBox>) -> Box<LineBox> {
        self.borrow_mut().boxes.push(element);
        self.clone()
    }
}

pub trait PaintElement: Stylable {
    fn draw(&self, target: &mut Tekenen, context: &FormattingInfo, space: Vec2);
}

#[derive(Debug)]
pub struct PainterTree {
    pub margin_box: Rect,
    pub border_box: Rect,
    pub padding_box: Rect,
    pub content_box: Rect,
    pub element: Box<dyn PaintElement>,
    pub context: FormattingInfo,
    pub children: Vec<PainterTree>
}


impl Display for PainterTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);

        writeln!(f, "{:width$}{}", "", self.element.get_name())?;
        let width = width + 2;
        writeln!(f, "{:width$}content: {}", "", self.content_box)?;

        let width = width + 4;
        for child in self.children.iter() {
            write!(f, "{:width$}", child)?;
        }

        Ok(())
    }
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

    pub fn translate(&mut self, distance: Vec2) {
        self.margin_box.position += distance;
        self.border_box.position += distance;
        self.padding_box.position += distance;
        self.content_box.position += distance;
    }
}