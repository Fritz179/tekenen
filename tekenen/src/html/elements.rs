pub mod div;
use std::{cell::{Ref, RefCell}, fmt::{Debug, Display}, rc::Rc};

pub use div::Div;

// pub mod slider;
// pub use slider::Slider;

pub mod text_fragment;
pub use text_fragment::TextNode;

pub mod p;
pub use p::P;

use crate::{math::{IndefRange, Vec2}, platform::Event, shapes::rect::Rect, DrawableSurface, SurfaceView};


use super::{style::{CSSDisplay, FormattingInfo, Style}, tree::{Tree, TreeData}};

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
pub enum BlockFormattingContextType {
    Block,
    Flex,
    Grid,
    Table,
}

impl BlockFormattingContextType {
    pub fn generate(&self) -> Box<dyn BlockFormattingContext> {
        match self {
            Self::Block => Box::new(BlockBlockFormattingContext::new()),
            // Self::Flex => Box::new(FlexFormattingContext::new()),
            Self::Flex => todo!(),
            Self::Grid => todo!(),
            Self::Table => todo!(),
        }
    }
}

pub trait LayoutBox: Stylable {
    fn get_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange>;

    // Get space constraints for layouting
    fn get_inner_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange>;

    // Used for layouting
    fn get_width_from_height(&self, height: i32, context: &FormattingInfo) -> i32;
    fn get_height_from_width(&self, width: i32, context: &FormattingInfo) -> i32;

    fn get_painter(self: Rc<Self>, content_box: Rect, context: &FormattingInfo) -> Rc<dyn PaintElement>;

    fn is_inline(&self) -> bool {
        todo!();
    }

    fn go_inline_yourself(&self, formatter: &InlineFormattingContext, info: &FormattingInfo) -> Vec<(Rc<LineBox>, Rc<dyn LayoutBox>)>;
}


// A block-level box is a box that participates in a block formatting context
pub trait BlockLayoutBox: LayoutBox {

}

// An inline-level box is a box that participates in an inline formatting context
pub trait InlineLayoutBox: LayoutBox {
    fn split_into_line(&self, formatter: InlineFormattingContext);
}

// This is what the tree is made of
// Each children can either be a Block or Inline level box
// Formatting Context are not limited to this two (Flex, Grid, Table...)
#[derive(Debug)]
pub struct LayoutNode {
    /// Anonyous nodes don't have a LayoutBox
    element: Option<Rc<dyn LayoutBox>>,
    children_are_inline: RefCell<bool>,

    tree_data: TreeData<LayoutNode>,
}

impl Tree for LayoutNode {
    fn get_data(&self) -> &TreeData<Self> {
        &self.tree_data
    }
}

impl LayoutNode {
    pub fn new(element: Option<Rc<dyn DomElement>>) -> Rc<LayoutNode> {
        let node = Rc::new(Self {
            element: element.clone().map(|element| element.get_layout_box()),
            children_are_inline: RefCell::new(false),
            tree_data: TreeData::new()
        });

        if let Some(element) = element {
            if let Some(children) = element.get_dom_children() {
                for child in children.borrow().iter() {
                    node.clone().insert_node_into_inline_or_block_ancestor(LayoutNode::new(Some(child.clone())));
                }
            }
        }

        node
    }

    fn is_anonymous(&self) -> bool {
        self.element.is_none()
    }

    // https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_display/Block_formatting_context

    fn creates_block_formatting_context(&self) -> bool {
        let style = self.element.as_ref().unwrap().get_style().borrow();

        // TODO: The root element of the document (<html>).
    
        // Floats (elements where float isn't none).
        if !style.float.is_none() {
            return true
        }

        // TODO: Absolutely positioned elements (elements where position is absolute or fixed).
        // TODO: Inline-blocks (elements with display: inline-block).
        // TODO: Table cells (elements with display: table-cell, which is the default for HTML table cells).
        // TODO: Table captions (elements with display: table-caption, which is the default for HTML table captions).
        // TODO: Anonymous table cells implicitly created by the elements with display: table, table-row, table-row-group, table-header-group, table-footer-group (which is the default for HTML tables, table rows, table bodies, table headers, and table footers, respectively), or inline-table.
        // TODO: Block elements where overflow has a value other than visible and clip.
        // TODO: display: flow-root.
        // TODO: Elements with contain: layout, content, or paint.

        if style.display.is_flex_inside() || style.display.is_grid_inside() || style.display.is_table_inside(){
            return false
        }
        
        if let Some(parent) = self.get_parent() {
            // Flex items (direct children of the element with display: flex or inline-flex) if they are neither flex nor grid nor table containers themselves.
            if parent.element.as_ref().unwrap().get_style().borrow().display.is_flex_inside() {
                return true
            }

            // Grid items (direct children of the element with display: grid or inline-grid) if they are neither flex nor grid nor table containers themselves.
        }
        
        // TODO: Multicol containers (elements where column-count or column-width isn't auto, including elements with column-count: 1).
        // TODO: column-span: all should always create a new formatting context, even when the column-span: all element isn't contained by a multicol container (Spec change, Chrome bug).
    
        false
    }

    fn type_of_formatting_context_to_generate(&self) -> Option<BlockFormattingContextType> {
        let display = &self.element.as_ref().unwrap().get_style().borrow().display;

        if display.is_flex_inside() {
            return Some(BlockFormattingContextType::Flex)
        }

        if self.creates_block_formatting_context() {
            return Some(BlockFormattingContextType::Block)
        }

        None
    }

    fn get_style(&self) -> Ref<'_, Style> {
        self.element.as_ref().unwrap().get_style().borrow()
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
            if !self.element.as_ref().unwrap().is_inline() && child.element.as_ref().unwrap().is_inline() {
                writeln!(f, "{:width$}InlineContext", "",)?;
            }
    
            if child.element.as_ref().unwrap().is_inline() {
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

    /// keep invariance that all children are all inline or all block
    /// Insert an anonymous block if needed
    fn insertion_parent_for_inline_node(self: Rc<LayoutNode>) -> Rc<LayoutNode> {
        let display = self.display();

        if display.is_inline_outside() && display.is_flow_inside() {
            drop(display);
            return self
        }

        // check current last child
        if let Some(child) = self.last_child() {
            if *child.children_are_inline.borrow() {
                return child
            }
        }

        // Anonymous block
        let anonymous_box = LayoutNode::new(None);
        self.append_child(anonymous_box.clone());
        anonymous_box
    }

    fn insertion_parent_for_block_node(self: Rc<LayoutNode>) -> Rc<LayoutNode> {
        self
    }

    fn insert_node_into_inline_or_block_ancestor(self: Rc<LayoutNode>, child: Rc<LayoutNode>) {
        // let display = &node.display();

        if child.element.as_ref().unwrap().is_inline() {
            let insertion = self.clone().insertion_parent_for_inline_node();
            insertion.append_child(child.clone());
            insertion.children_are_inline.replace(true);
        } else {
            let insertion = self.clone().insertion_parent_for_block_node();
            insertion.append_child(child.clone());
            insertion.children_are_inline.replace(false);
        }
    }
}

pub trait FormattingContext {
    fn get_line(&self, info: &FormattingInfo) -> (Rc<LineBox>, bool);
    fn get_new_line(&self, info: &FormattingInfo) -> (Rc<LineBox>, bool);
    fn run(&self, node: &Rc<LayoutNode>, info: &FormattingInfo) -> Option<PainterTree>;

    fn layout_inside(&self, node: &Rc<LayoutNode>, info: &FormattingInfo) -> Option<PainterTree> {
        if let Some(context) = node.type_of_formatting_context_to_generate() {
            context.generate().run(node, info)
        } else {
            self.run(node, info)
        }
    }
}

pub trait BlockFormattingContext: FormattingContext {

}

pub struct BlockBlockFormattingContext {
    
}

impl Default for BlockBlockFormattingContext {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockBlockFormattingContext {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl BlockFormattingContext for BlockBlockFormattingContext {
    
}

impl FormattingContext for BlockBlockFormattingContext {
    fn get_line(&self, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        todo!()
    }

    fn get_new_line(&self, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        todo!()
    }

    fn run(&self, node: &Rc<LayoutNode>, info: &FormattingInfo) -> Option<PainterTree> {
        assert!(!*node.children_are_inline.borrow());

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

impl BlockBlockFormattingContext {
    fn run_parent(&self, node: &Rc<LayoutNode>, info: &FormattingInfo) -> Option<PainterTree> {
        // creates a new formatting context
        if let Some(new_context) = node.type_of_formatting_context_to_generate() {
            return new_context.generate().run(node, info)
        }

        // Apply CSS Definite sizes
        let mut available_size = info.containing_block;

        if let Some(width) = node.element.as_ref().unwrap().get_style().borrow().width.solve(info) {
            available_size.size.x = width;
        }

        if let Some(height) = node.element.as_ref().unwrap().get_style().borrow().height.solve(info) {
            available_size.size.y = height;
        }

        // Get inner size for children
        let bounding = node.element.as_ref().unwrap().get_style().borrow().get_total_computed_boudning(info);
        let available_content_rect = available_size - bounding.clone();

        // Walk through all children
        let mut children = vec![];

        let start_y = available_content_rect.position.y;
        let mut current_y = start_y;

        for child in node.tree_data.iter() {
            let child_info = FormattingInfo {
                containing_block: Rect::new_vec(Vec2::new(available_content_rect.position.x, current_y), available_content_rect.size),
            };

            // TODO: Can be better
            if *child.children_are_inline.borrow() {
                let inline = InlineFormattingContext::new(self);

                let child = inline.run(&child, &child_info).unwrap();
                current_y += child.margin_box.size.y;
                children.push(child);
            } else {
                let child = self.run(&child, &child_info).unwrap();
                current_y += child.margin_box.size.y;
                children.push(child);
            }
        }

        let inner_height = current_y - start_y;
        let outer_height = inner_height + bounding.top + bounding.bottom;
        let outer_width = available_size.size.x;

        // println!("Inner: {}, Top: {}, Bottom: {}", inner_height, bounding.top, bounding.bottom);

        let margin_box = Rect::new_vec(info.containing_block.position, Vec2::new(outer_width, outer_height));
        let content_box = margin_box - bounding;

        let element = node.element.clone().unwrap().get_painter(content_box, info);

        Some(PainterTree {
            margin_box,
            border_box: content_box,
            padding_box: content_box,
            content_box,
            children,
            element: Some(element),
            context: info.clone()
        })
    }

    fn run_leaf(&self, node: &LayoutNode, element: Rc<dyn LayoutBox>, info: &FormattingInfo) -> PainterTree {

        // get CSS restrictions
        let size_constraint = element.get_style().borrow().get_size_contraint(info);

        // Get inner width
        let boudning = element.get_style().borrow().get_total_computed_boudning(info);
        let outer_width = info.containing_block.size.x;
        let inner_width = outer_width - boudning.left - boudning.right;

        // Get inner height
        let inner_height = element.get_height_from_width(inner_width, info);
        
        // Get outer dimensions
        let outer_height = inner_height + boudning.top + boudning.bottom;
        let margin_box = Rect::new_vec(info.containing_block.position, Vec2::new(outer_width, outer_height));
        let content_box = margin_box - boudning;

        let element = element.clone().get_painter(content_box, info);

        dbg!(content_box);

        PainterTree {
            margin_box: content_box,
            border_box: content_box,
            padding_box: content_box,
            content_box,
            children: vec![],
            element: Some(element),
            context: info.clone()
        }
    }
}


pub struct InlineFormattingContext<'a> {
    /// All the lines
    pub lines: RefCell<Vec<Rc<LineBox>>>,
    // inline-element, Vec of each child piece Vec<containing line, piece>
    // elements: Vec<(Rc<LayoutNode>, Vec<(Rc<LineBox>, Rc<dyn LayoutBox>)>)>,
    pub parent: &'a BlockBlockFormattingContext
}

impl<'a>  InlineFormattingContext<'a>  {
    pub fn new(parent: &'a BlockBlockFormattingContext) -> Self {
        Self {
            lines: RefCell::new(vec![]),
            parent
        }
    }
}

impl<'a> FormattingContext for InlineFormattingContext<'a> {
    fn get_line(&self, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        if self.lines.borrow().is_empty() {
            return self.get_new_line(info);
        }

        return (self.lines.borrow().last().unwrap().clone(), false);
    }

    fn get_new_line(&self, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
        let line = Rc::new(LineBox {
            boxes: RefCell::new(Vec::new()),
            max_width: info.containing_block.size.x,
            width: 0,
            height: RefCell::new(None),
            y: RefCell::new(None),
        });

        self.lines.borrow_mut().push(line.clone());

        (line, true)
    }

    fn run(&self, node: &Rc<LayoutNode>, info: &FormattingInfo) -> Option<PainterTree> {
        assert!(*node.children_are_inline.borrow());

        let mut elements = vec![];
        
        for child in node.tree_data.iter() {
            let lines = child.element.as_ref().unwrap().go_inline_yourself(self, info);
            elements.push((node, lines));
        }

        // We are closing the InlineFormattingContext

        let start_y = info.containing_block.position.y;
        let mut current_y = start_y;

        // Calculate the height of each line
        for line in self.lines.borrow().iter() {
            let boxes = &line.boxes.borrow();

            assert!(boxes.len() == 1, "We only support one box per line");

            let box_ = &boxes[0];
            let height = box_.get_height_from_width(info.containing_block.size.x, info);

            // Update info about lines
            *line.y.borrow_mut() = Some(current_y);
            *line.height.borrow_mut() = Some(height);
            current_y += height;
        }

        let children = elements.iter().map(|(node, pieces)| {
            // TODO: Everything
            let mut inner = Vec::new();

            assert!(!pieces.is_empty());

            let mut min_y = i32::MAX;
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
                    element: Some(piece),
                    context: info.clone(),
                    children: vec![]
                });
            }

            let element_block = Rect::new_vec(
                Vec2::new(info.containing_block.position.x, min_y), 
                Vec2::new(info.containing_block.size.x, max_y - min_y)
            );

            let element = node.element.as_ref().unwrap().clone().get_painter(element_block, info);

            PainterTree {
                margin_box: element_block,
                border_box: element_block,
                padding_box: element_block,
                content_box: element_block,
                element: Some(element),
                context: info.clone(),
                children: inner
            }
        }).collect();

        let margin_box = Rect::new_vec(info.containing_block.position, Vec2::new(info.containing_block.size.x, current_y - start_y));

        Some(PainterTree {
            margin_box: margin_box,
            border_box: margin_box,
            padding_box: margin_box,
            content_box: margin_box,
            element: None,
            context: info.clone(),
            children
        })
    }
}

// struct FlexFormattingContext {

// }

// impl FlexFormattingContext {
//     fn new() -> Self {
//         Self {

//         }
//     }

//     fn flex_row(&self, node: &Rc<LayoutNode>, context: &FormattingInfo) -> Vec<PainterTree> {
//         let width = self.determine_width(node, context);

//         let child_context = FormattingInfo {
//             containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(width, 0))
//         };

//         struct ChildData {
//             child: Rc<LayoutNode>,
//             min_width: i32,
//             curr_width: i32,
//             height: i32
//         }

//         // 1. Calculate desired sizes 
//         let mut min_curr_width_height: Vec<ChildData> = node.tree_data.iter().map(|child| {
//             let size = child.element.unwrap().get_min_max_margin_area(&child, &child_context);
//             let height = child.get_height_from_width(&child, size.x.get_max().unwrap(), &child_context);

//             ChildData {
//                 child,
//                 min_width: size.x.get_min().unwrap(),
//                 curr_width: size.x.get_max().unwrap(),
//                 height
//             }
//         }).collect();

//         let desired_size: i32 = min_curr_width_height.iter().map(|(_, _, curr, _)| curr).sum();
//         let error = width - desired_size;


//         // 2. Distribute error
//         if error > 0 {
//             // Grow all elements by same ammout

//             // TODO: Exact correction is float
//             let correction = error / min_curr_width_height.len() as i32;
//             min_curr_width_height.iter_mut().for_each(|(child, _, curr, height)| {
//                 *curr += correction;
//                 let child = child.borrow();
//                 *height = child.get_style().get_height_from_width(&child, *curr, &child_context);
//             })

//         } else if error < 0 {
//             loop {
                
//                 // Error to distribute
//                 let desired_size: i32 = min_curr_width_height.iter().map(|(_, _, curr, _)| curr).sum();
//                 let error = width - desired_size;

//                 // Space distributed
//                 if error >= 0 {
//                     break
//                 }
                
//                 // how many can adjust
//                 let unfrozen = min_curr_width_height.iter().filter(|(_, min, curr, _)| min != curr).count() as i32;

//                 // no one can be adjusted
//                 if unfrozen == 0 {
//                     break
//                 }

//                 // positive correction
//                 let correction = -error / unfrozen;

//                 min_curr_width_height.iter_mut().filter(|(child, min, curr, height)| min != curr)
//                 .for_each(|(child, min, curr, height)| {
//                     if *curr - correction >= *min {
//                         *curr -= correction;
//                     } else {
//                         *curr = *min
//                     }

//                     let child = child.borrow();
//                     *height = child.get_style().get_height_from_width(&child, *curr, &child_context);
//                 })
//             }
//         }

//         let height = *min_curr_width_height.iter().map(|(_, _, _, curr)| curr).max().unwrap();
//         let real_context = Context {
//             containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(width, height))
//         };
//         let mut offset = Vec2::new(0, 0);

//         // Get Painters
//         min_curr_width_height.into_iter().map(|(child, _, curr, height)| {
//             let child_b = child.borrow();
//             let bb = child_b.get_style();

//             let margin_box = Rect::new_vec(context.containing_block.position + offset, Vec2::new(curr, height));
//             let content_box = margin_box - bb.get_total_computed_boudning(context);

//             let painter = bb.get_painter(Rc::clone(&child), content_box, real_context.clone());
//             offset.x += curr;
//             painter
//         }).collect()
//     }

//     // fn flex_column(&self, context: &Context, size: Vec2) -> Vec<PainterTree> {
//     //     let width = self.determine_width(context);

//     //     println!("COLUMN: Width: {}", width);

//     //     let height_range = self.determine_height_range(context);
        
//     //     let child_context = Context {
//     //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(width, 0))
//     //     };
//     //     // 1. Calculate desired sizes 

//     //     // child, minimum height, curr = maximum_height, width
//     //     let mut min_curr_height_width: Vec<(Rc<RefCell<dyn Element>>, i32, i32, i32)> = self.children.iter().map(|child| {
//     //         let child_b = child.borrow();
//     //         let bb = child_b.get_style();

//     //         let min_max = bb.get_min_max_margin_area(&child_b, &child_context);
//     //         let child_width = bb.get_width_from_height(&child_b, min_max.y.get_max().unwrap(), &child_context);

//     //         (
//     //             Rc::clone(child), 
//     //             min_max.y.get_min().unwrap(), 
//     //             min_max.y.get_max().unwrap().max(bb.get_height_from_width(&child_b, width, context)), 
//     //             child_width
//     //         )
//     //     }).collect();

//     //     // for (_, min, curr, width) in &min_curr_height_width {
//     //     //     println!("COLUMN: Min: {}, Curr: {}, Width: {}", min, curr, width)
//     //     // }

//     //     let desired_size: i32 = min_curr_height_width.iter().map(|(_, _, curr, _)| curr).sum();
//     //     let height = height_range.constrain(desired_size);
//     //     let error = height - desired_size;

//     //     // 2. Distribute error only if to short
//     //     if error < 0 {
//     //         loop {
                
//     //             // Error to distribute
//     //             let desired_size: i32 = min_curr_height_width.iter().map(|(_, _, curr, _)| curr).sum();
//     //             let error = height - desired_size;

//     //             // Space distributed
//     //             if error >= 0 {
//     //                 break
//     //             }
                
//     //             // how many can adjust
//     //             let unfrozen = min_curr_height_width.iter().filter(|(_, min, curr, _)| min != curr).count() as i32;

//     //             // no one can be adjusted
//     //             if unfrozen == 0 {
//     //                 break
//     //             }

//     //             // positive correction
//     //             let correction = -error / unfrozen;

//     //             min_curr_height_width.iter_mut().filter(|(child, min, curr, width)| min != curr).for_each(|(child, min, curr, width)| {
//     //                 if *curr - correction >= *min {
//     //                     *curr -= correction;
//     //                 } else {
//     //                     *curr = *min
//     //                 }

//     //                 let child = child.borrow();
//     //                 *width = child.get_style().get_width_from_height(&child, *curr, &child_context);
//     //             })
//     //         }
//     //     }

//     //     let height = *min_curr_height_width.iter().map(|(_, _, curr, _)| curr).max().unwrap();
//     //     let real_context = Context {
//     //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(width, height))
//     //     };

//     //     let mut offset = Vec2::new(0, 0);

//     //     min_curr_height_width.into_iter().map(|(child, _, curr, _)| {
//     //         let child_b = child.borrow();
//     //         let bb = child_b.get_style();
            
//     //         let margin_box = Rect::new_vec(context.containing_block.position + offset, Vec2::new(width, curr));
//     //         let content_box = margin_box - bb.get_total_computed_boudning(context);

//     //         let painter = bb.get_painter(Rc::clone(&child), content_box, real_context.clone());

//     //         offset.y += curr;
//     //         painter
//     //     }).collect()
//     // }

//     fn determine_width(&self, node: &Rc<LayoutNode>, context: &FormattingInfo) -> i32 {
//         let style = node.get_style();

//         // Determine the available main and cross space for the flex items. For
//         // each dimension, if that dimension of the flex container’s content box
//         // is a definite size, use that; if that dimension of the flex container
//         // is being sized under a min or max-content constraint, the available
//         // space in that dimension is that constraint; otherwise, subtract the
//         // flex container’s margin, border, and padding from the space available
//         // to the flex container in that dimension and use that value.

//         // 1. is definite
//         if let Some(value) = style.width.solve(context) {
//             return value
//         }

//         // 2. is under min/max constraint
//         let min = style.min_width.solve(context);
//         let max = style.max_width.solve(context);

//         if min.is_some() || max.is_some() {
//             return IndefRange::new_min_priority(min, max).constrain(context.containing_block.size.x)
//         }
        
//         // 3. margin, border and padding
//         context.containing_block.size.x - style.get_total_computed_boudning(context).get_total_width()
//     }

//     fn determine_height_range(&self, node: &Rc<LayoutNode>, context: &FormattingInfo) -> IndefRange {
//         let style = node.get_style();

//         // 1. is definite
//         if let Some(value) = style.height.solve(context) {
//             return IndefRange::new_definite(value)
//         }

//         // 2. is under min/max constraint
//         let min = style.min_height.solve(context);
//         let max = style.max_height.solve(context);

//         IndefRange::new_option(min, max)
//     }
// }

// impl FormattingContext for FlexFormattingContext {
//     fn get_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
//         todo!()
//     }

//     fn get_new_line(&mut self, context: &dyn FormattingContext, info: &FormattingInfo) -> (Rc<LineBox>, bool) {
//         todo!()
//     }
// }

// impl BlockFormattingContext for FlexFormattingContext {
//     fn run(&self, node: &Rc<LayoutNode>, info: &FormattingInfo) -> Option<PainterTree> {
//         let style = node.element.as_ref().borrow().unwrap().get_style().borrow();
//         assert!(style.display.is_flex_inside());

//         let width = self.determine_width(node, info);

//         match style.flex_direction {
//             CSSFlexDirection::Row => self.flex_row( size),
//             _ => todo!()
//         }
//     }
// }

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
    fn draw(&self, target: &mut SurfaceView, context: &FormattingInfo, space: Vec2);
}

#[derive(Debug)]
pub struct PainterTree {
    pub margin_box: Rect,
    pub border_box: Rect,
    pub padding_box: Rect,
    pub content_box: Rect,
    pub element: Option<Rc<dyn PaintElement>>,
    pub context: FormattingInfo,
    pub children: Vec<PainterTree>
}


impl Display for PainterTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);

        writeln!(f, "{:width$}{}", "", self.element.as_ref().map_or("ANONYMOUS".to_string(), |el| el.get_name()))?;
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
    pub fn paint(&self, target: &mut SurfaceView) {
        // let element = self.element.borrow();
        // let style = element.get_style();

        // let bg_color = style.background_color.solve(&self.context);

        // if bg_color[3] > 0 {
        //     target.set_translation_vec(self.border_box.position);
        //     target.rect_vec(Vec2::zero(), self.border_box.size, bg_color);
        // }

        todo!("Read next line");
        // target.set_translation_vec(self.content_box.position);

        if let Some(element) = self.element.as_ref() {
            element.draw(target, &self.context, self.content_box.size);
        }

        for element in self.children.iter() {
            element.paint(target);
        }
    }
}