pub mod div;
use std::{borrow::Borrow, cell::{Ref, RefCell}, fmt::{Debug, Display}, ops::Deref, rc::{Rc, Weak}};

pub use div::Div;

// pub mod slider;
// pub use slider::Slider;

pub mod textFragment;
pub use textFragment::TextNode;

pub mod p;
pub use p::P;

use crate::{math::{IndefRange, Vec2}, platform::Event, shapes::rect::Rect, Draw, Tekenen};


use super::{style::{CSSDisplay, CSSDisplayInside, FormattingInfo, Style}, tree::{Tree, TreeData}};

pub trait Stylable: Debug {
    fn get_style(&self) -> &RefCell<Style>;
    fn get_name(&self) -> String;
}

// Every HTML element has to implement this Trait
pub trait DomElement: Stylable {
    // React to event
    fn event(&mut self, event: Event);

    // Called once before layout and draw
    fn update(&mut self);

    // Implement for default behavior
    fn get_dom_children(&self) -> Option<&RefCell<Vec<Rc<dyn DomElement>>>>;

    // Implement to get a Layout/Box Tree
    fn get_layout_box(self: Rc<Self>) -> Rc<dyn LayoutBox>;
}

impl Display for dyn DomElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);

        writeln!(f, "{:width$}{}", "", self.get_name())?;

        let width = width + 4;
        if let Some(children) = self.get_dom_children() {
            for child in children.borrow().iter() {
                write!(f, "{:width$}", child)?;
            }
        }

        Ok(())
    }
}

// Every entry in the Layout/Box tree has to implemnet this 
#[derive(Debug, PartialEq)]
pub enum FormattingContextType {
    Block,
    Inline,
    Flex,
    Grid,
    Table,
    None
}

pub trait LayoutBox: Stylable {
    fn get_min_max_content(&self, context: FormattingInfo) -> Vec2<IndefRange>;

    // Get space constraints for layouting
    fn get_inner_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange>;

    // Used for layouting
    fn get_width_from_height(&self, height: i32, context: &FormattingInfo) -> i32;
    fn get_height_from_width(&self, width: i32, context: &FormattingInfo) -> i32;

    fn get_painter(self: Rc<Self>, content_box: Rect, context: &FormattingInfo) -> Rc<dyn PaintElement>;

    fn is_inline(&self) -> bool {
        todo!();
    }

    // https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_display/Block_formatting_context
    fn creates_block_formatting_context(&self) -> bool {
        false
    }

    fn type_of_formatting_context_to_generate(&self) -> Option<FormattingContextType> {
        let display = &self.get_style().borrow().display;

        if display.is_flex_inside() {
            return Some(FormattingContextType::Flex)
        }

        None
    }

    fn create_formatting_context_if_needed(&self) -> Option<ContextDecision> {
        None
    }

    fn go_inline_yourself(&self, formatter: &mut InlineFormattingContext, context: &dyn FormattingContext, info: &FormattingInfo) 
        -> Vec<(Rc<LineBox>, Rc<dyn LayoutBox>)>;
}


// A block-level box is a box that participates in a block formatting context
pub trait BlockLayoutBox: LayoutBox {

}

// An inline-level box is a box that participates in an inline formatting context
pub trait InlineLayoutBox: LayoutBox {
    fn split_into_line(&self, formatter: InlineFormattingContext);
}

#[derive(Debug, PartialEq)]
pub enum ContextDecision {
    BlockContext,
    InlineContext,
    InlineElement,
    FlexContext,
    None
}

impl ContextDecision {
    fn create_formatting_context_if_needed(&self) -> Option<Box<dyn BlockFormattingContext>> {
        match self {
            Self::BlockContext => Some(Box::new(BlockBlockFormattingContext::new())),
            Self::FlexContext => todo!(),
            Self::None | Self::InlineContext | Self:: InlineElement => None
        }
    }
}

// This is what the tree is made of
// Each children can either be a Block or Inline level box
// Formatting Context are not limited to this two (Flex, Grid, Table...)
#[derive(Debug)]
pub struct LayoutNode {
    /// Anonyous nodes don't have a LayoutBox
    element: Option<Rc<dyn LayoutBox>>,
    context: RefCell<ContextDecision>,
    children_are_inline: RefCell<bool>,

    tree_data: TreeData<LayoutNode>,
}

impl Tree for LayoutNode {
    fn get_data(&self) -> &TreeData<Self> {
        &self.tree_data
    }
}

impl LayoutNode {
    pub fn new(element: Rc<dyn DomElement>) -> Rc<LayoutNode> {
        let node = Rc::new(Self {
            element: Some(element.clone().get_layout_box()),
            context: RefCell::new(ContextDecision::None),
            children_are_inline: RefCell::new(false),
            tree_data: TreeData::new()
        });

        element.get_dom_children().map(|children| {
            for child in children.borrow().iter() {
                node.clone().insert_node_into_inline_or_block_ancestor(LayoutNode::new(child.clone()));
            }
        });

        node
    }
}

/*

div
    BlockContext
        p
            InlineContext
                Text
                Bold
                Text
            block
        div
            el 
            el2

*/

impl Display for LayoutNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut width = f.width().unwrap_or(0);

        writeln!(f, "{:width$}{}", "", self.element.as_ref().map(|element| element.get_name()).unwrap_or("Anon".to_string()))?;

        width += 4;
        for child in self.tree_data.iter() {
            if *child.context.borrow() == ContextDecision::InlineContext {
                writeln!(f, "{:width$}InlineContext", "",)?;
            }
    
            if *child.context.borrow() == ContextDecision::InlineContext || *child.context.borrow() == ContextDecision::InlineElement {
                write!(f, "{:width$}", child, width = width + 4)?;
            } else {
                write!(f, "{:width$}", child)?;
            }
        }

        Ok(())
    }
}

impl LayoutNode {
    fn display(&self) -> Ref<'_, CSSDisplay> {
        Ref::map(self.element.as_ref().unwrap().get_style().borrow(), |style| &style.display)
    }

    fn insertion_parent_for_inline_node(self: Rc<LayoutNode>) -> Rc<LayoutNode> {
        let display = self.display();

        if display.is_inline_outside() && display.is_flow_inside() {
            drop(display);
            return self
        }

        drop(display);
        self
        // todo!()
    }

    fn insertion_parent_for_block_node(self: Rc<LayoutNode>) -> Rc<LayoutNode> {
        self
    }

    fn insert_node_into_inline_or_block_ancestor(self: Rc<LayoutNode>, node: Rc<LayoutNode>) {
        // let display = &node.display();

        // if display.is_inline_outside() {
        if node.element.as_ref().unwrap().is_inline() {
            let insertion = self.clone().insertion_parent_for_inline_node();
            insertion.append_child(node.clone());
            insertion.children_are_inline.replace(true);
        } else {
            let insertion = self.clone().insertion_parent_for_block_node();
            insertion.append_child(node.clone());
            insertion.children_are_inline.replace(false);
        }
        if node.element.as_ref().unwrap().is_inline() {
            *node.context.borrow_mut() = if let Some(last) = self.last_child() {
                if *last.context.borrow() != ContextDecision::InlineElement && *last.context.borrow() != ContextDecision::InlineContext {
                    ContextDecision::InlineContext
                } else {
                    ContextDecision::InlineElement
                }
            } else {
                ContextDecision::InlineContext
            };
        }
    }
}

pub trait FormattingContext {
    fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool);
    fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool);
}

pub trait BlockFormattingContext: FormattingContext {
    fn run(&self, node: &LayoutNode, info: &FormattingInfo) -> Option<PainterTree>;
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
    fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        todo!()
    }

    fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        todo!()
    }
}

impl BlockBlockFormattingContext {
    fn run_parent(&self, node: &LayoutNode, info: &FormattingInfo) -> Option<PainterTree> {
        // Apply CSS Definite sizes
        let mut available_size = info.containing_block;

        if let Some(width) = node.element.as_ref().unwrap().get_style().borrow().width.solve(info) {
            available_size.size.x = width;
        }

        if let Some(height) = node.element.as_ref().unwrap().get_style().borrow().height.solve(info) {
            available_size.size.y = height;
        }

        // Get inner size for children
        let bounding = node.element.as_ref().unwrap().get_style().borrow().get_total_computed_boudning(&info);
        let available_content_rect = available_size - bounding.clone();

        // Walk through all children
        let mut inline = None;
        let mut children = vec![];

        let start_y = available_content_rect.position.y;
        let mut current_y = start_y;

        for child in node.tree_data.iter() {
            let child_info = FormattingInfo {
                containing_block: Rect::new_vec(Vec2::new(available_content_rect.position.x, current_y), available_content_rect.size)
            };

            // TODO: Can be better
            if *child.context.borrow() == ContextDecision::InlineElement || *child.context.borrow() == ContextDecision::InlineContext {
                let inline = inline.get_or_insert(InlineFormattingContext::new());

                inline.add_inline(child, self, info);
                // println!("Inlining: {child:?}");
            } else {
                if let Some(inline) = inline {
                    let (mut child, height) = inline.run( self, &child_info);
                    current_y += height;
                    children.append(&mut child);
                }

                inline = None;

                let child = self.run(child.as_ref(), &child_info).unwrap();
                current_y += child.margin_box.size.y;
                children.push(child);
            }
        }

        if let Some(inline) = inline {
            let child_info = FormattingInfo {
                containing_block: Rect::new_vec(Vec2::new(available_content_rect.position.x, current_y), available_content_rect.size)
            };

            let (mut child, height) = inline.run( self, &child_info);
            current_y += height;
            children.append(&mut child);
        }

        let inner_height = current_y - start_y;
        let outer_height = inner_height + bounding.top + bounding.bottom;
        let outer_width = available_size.size.x;

        // println!("Inner: {}, Top: {}, Bottom: {}", inner_height, bounding.top, bounding.bottom);

        let margin_box = Rect::new_vec(info.containing_block.position, Vec2::new(outer_width, outer_height));
        let content_box = margin_box - bounding;

        let element = node.element.clone().unwrap().get_painter(content_box, &info);

        Some(PainterTree {
            margin_box: margin_box,
            border_box: content_box,
            padding_box: content_box,
            content_box,
            children,
            element,
            context: info.clone()
        })
    }

    fn run_leaf(&self, node: &LayoutNode, element: Rc<dyn LayoutBox>, info: &FormattingInfo) -> PainterTree {

        // get CSS restrictions
        let size_constraint = element.get_style().borrow().get_size_contraint(&info);

        // Get inner width
        let boudning = element.get_style().borrow().get_total_computed_boudning(&info);
        let outer_width = info.containing_block.size.x;
        let inner_width = outer_width - boudning.left - boudning.right;

        // Get inner height
        let inner_height = element.get_height_from_width(inner_width, &info);
        
        // Get outer dimensions
        let outer_height = inner_height + boudning.top + boudning.bottom;
        let margin_box = Rect::new_vec(info.containing_block.position, Vec2::new(outer_width, outer_height));
        let content_box = margin_box - boudning;

        let element = element.clone().get_painter(content_box, &info);

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

impl BlockFormattingContext for BlockBlockFormattingContext {
    fn run(&self, node: &LayoutNode, info: &FormattingInfo) -> Option<PainterTree> {
        if let Some(context) = node.context.borrow().create_formatting_context_if_needed() {
            return context.run(node, info);
        }

        if node.has_children() {
            return self.run_parent(node, info)
        }

        // run leaf if present
        assert!(node.element.is_some());

        node.element.as_ref().map(|element| {
            self.run_leaf(node, element.clone(), info)
        })
    }
}


pub struct InlineFormattingContext {
    /// All the lines
    pub lines: Vec<Rc<LineBox>>,
    /// inline-element, Vec of each child piece Vec<containing line, piece>
    elements: Vec<(Rc<LayoutNode>, Vec<(Rc<LineBox>, Rc<dyn LayoutBox>)>)>,
}

impl InlineFormattingContext {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            elements: vec![]
        }
    }
}

impl FormattingContext for  InlineFormattingContext {
    fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        if self.lines.is_empty() {
            return self.get_new_line(context, info);
        }

        return (self.lines.last().unwrap().clone(), false);
    }

    fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        let line = Rc::new(LineBox {
            boxes: RefCell::new(Vec::new()),
            max_width: info.containing_block.size.x,
            width: 0,
            height: RefCell::new(None),
            y: RefCell::new(None),
        });

        self.lines.push(line.clone());

        return (line, true);
    }
}

impl InlineFormattingContext {
    fn run(&self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Vec<PainterTree>, i32) {
        // We are closing the InlineFormattingContext

        let start_y = info.containing_block.position.y;
        let mut current_y = start_y;

        // Calculate the height of each line
        for line in self.lines.iter() {
            let boxes = &line.boxes.borrow();

            assert!(boxes.len() == 1, "We only support one box per line");

            let box_ = &boxes[0];
            let height = box_.get_height_from_width(info.containing_block.size.x, info);

            // Update info about lines
            *line.y.borrow_mut() = Some(current_y);
            *line.height.borrow_mut() = Some(height);
            current_y += height;
            // println!("Line height: {}", height);
        }

        let children = self.elements.iter().map(|(node, pieces)| {
            // TODO: Everything
            let mut inner = Vec::new();

            assert!(!pieces.is_empty());

            let mut min_y = 0;
            let mut max_y = 0;

            for (line, piece) in pieces.iter() {
                let y = line.y.borrow().unwrap();
                let height = line.height.borrow().unwrap();

                if y < min_y {
                    min_y = y;
                }

                if y + height > max_y {
                    max_y = y + height;
                }

                let child_block = Rect::new_vec(
                    Vec2::new(info.containing_block.position.x, y), 
                    Vec2::new(piece.get_width_from_height(16, info), height)
                );
    
                let piece = piece.clone().get_painter(child_block, info);
    
                inner.push(PainterTree {
                    margin_box: child_block,
                    border_box: child_block,
                    padding_box: child_block,
                    content_box: child_block,
                    element: piece,
                    context: info.clone(),
                    children: vec![]
                });
            }

            let element_block = Rect::new_vec(
                Vec2::new(info.containing_block.position.x, min_y), 
                Vec2::new(info.containing_block.size.x, max_y - min_y)
            );

            let element = node.element.as_ref().unwrap().clone().get_painter(element_block, &info);


            PainterTree {
                margin_box: element_block,
                border_box: element_block,
                padding_box: element_block,
                content_box: element_block,
                element,
                context: info.clone(),
                children: inner
            }
        }).collect();

        (children, current_y - start_y)
    }

    fn add_inline_parent(&mut self, node: Rc<LayoutNode>, context: &dyn FormattingContext, info: &FormattingInfo) {
        todo!()
    }
    
    fn add_inline(&mut self, node: Rc<LayoutNode>, context: &dyn FormattingContext, info: &FormattingInfo) {
        if node.has_children() {
            return self.add_inline_parent(node, context, info)
        }

        let children = node.element.as_ref().unwrap().go_inline_yourself(self, context, info);
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
    fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        todo!()
    }

    fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        todo!()
    }
}

impl BlockFormattingContext for FlexFormattingContext {
    fn run(&self, node: &LayoutNode, info: &FormattingInfo) -> Option<PainterTree> {
        todo!()
    }
}

// The line box is a horizontal stack of inline-level boxes
#[derive(Debug)]
pub struct LineBox {
    boxes: RefCell<Vec<Rc<dyn LayoutBox>>>,
    max_width: i32,
    width: i32,
    height: RefCell<Option<i32>>,
    y: RefCell<Option<i32>>
}

impl LineBox {
    pub fn add(&self, element: Rc<dyn LayoutBox>) {
        self.boxes.borrow_mut().push(element);
    }

    pub fn available_width(&self) -> i32 {
        self.max_width - self.width
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
    pub element: Rc<dyn PaintElement>,
    pub context: FormattingInfo,
    pub children: Vec<PainterTree>
}


impl Display for PainterTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);

        writeln!(f, "{:width$}{}", "", self.element.get_name())?;
        let width = width + 2;
        writeln!(f, "{:width$}content: {}", "", self.content_box)?;
        writeln!(f, "{:width$}padding: {}", "", self.padding_box)?;
        writeln!(f, "{:width$}border: {}", "", self.border_box)?;
        writeln!(f, "{:width$}margin: {}", "", self.margin_box)?;

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
}