use std::{any::Any, cell::{Ref, RefCell}, rc::Rc};

use crate::{math::{IndefRange, Vec2}, shapes::rect::Rect, ui::style::{CSSDisplay, CSSFlexDirection, LayoutContext}, Tekenen, Wrapper};

use super::{BlockLayoutBox, DomElement, LayoutBox, LayoutNode, PainterTree, Style};


/// A div is a flexbox
/// A div with a single element is a flexbox with a single element
#[derive(Debug)]
pub struct InnerDiv {
    pub style: Style,
    children: Vec<Box<dyn DomElement>>,
}

pub type Div = Wrapper<InnerDiv>;

impl Div {
    pub fn new(children: Vec<Box<dyn DomElement>>) -> Box<Self> {
        let mut style = Style::default();

        style.display = CSSDisplay::Block;

        Wrapper::wrap(InnerDiv {
            style,
            children
        })
    }

    pub fn new_fn(children: Vec<Box<dyn DomElement>>, fun: impl FnOnce(&mut InnerDiv)) -> Box<Self> {
        let mut style = Style::default();

        style.display = CSSDisplay::Block;

        let mut div = InnerDiv {
            style,
            children
        };

        fun(&mut div);

        Wrapper::wrap(div)
    }

    // pub fn new_vertical_flex(children: Vec<RcRefCell<dyn DomElement>>) -> RcRefCell<Self> {
    //     let mut style = Style::default();

    //     style.display = CSSDisplay::Flex;
    //     style.flex_direction = CSSFlexDirection::Column;

    //     Rc::new(RefCell::new(Div {
    //         style,
    //         children
    //     }))
    // }

    // pub fn new_horizontal_flex(children: Vec<RcRefCell<dyn DomElement>>) -> RcRefCell<Self> {
    //     let mut style = Style::default();

    //     style.display = CSSDisplay::Flex;
    //     style.flex_direction = CSSFlexDirection::Row;

    //     Rc::new(RefCell::new(Div {
    //         style,
    //         children
    //     }))
    // }
}


impl DomElement for Div {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn get_layout_box(&self) -> LayoutNode<dyn LayoutBox> {
        let node = match self.borrow().style.display {
            CSSDisplay::Block => LayoutNode::new(self.clone() as Box<dyn BlockLayoutBox>),
            CSSDisplay::Inline => todo!(),
            CSSDisplay::Flex => todo!(),
            CSSDisplay::None => todo!(),
        };

        for child in self.borrow().children.iter() {
            node.add_box(child.get_layout_box())
        }

        node
    }

    fn get_dom_children(&self) -> &Vec<Box<dyn DomElement>> {
        todo!()
    }


    // fn get_width_from_height(&self, height: i32, context: &LayoutContext) -> i32 {
    //     let child_context = LayoutContext {
    //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(self.determine_width(context), 0))
    //     };
        
    //     self.children.iter().map(|child| {
    //         let child = child.borrow();
    //         child.get_style().get_width_from_height(&child, height, &child_context)
    //     }).sum()
    // }

    // fn get_height_from_width(&self, width: i32, context: &LayoutContext) -> i32 {
    //     let child_context = LayoutContext {
    //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(width, 0))
    //     };

    //     self.children.iter().map(|child| {
    //         let child = child.borrow();
    //         child.get_style().get_height_from_width(&child, width, &child_context)
    //     }).sum()
    // }

    // fn get_inner_min_max_content(&self, context: &LayoutContext) -> Vec2<IndefRange> {
    //     let child_context = LayoutContext {
    //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(self.determine_width(context), 0))
    //     };

    //     let children: Vec<Vec2<IndefRange>> = self.children.iter().map(|child| {
    //         let child = child.borrow();
    //         child.get_style().get_min_max_margin_area(&child, &child_context)
    //     }).collect();


    //     match self.style.display {
    //         CSSDisplay::None => todo!(),
    //         CSSDisplay::Block => todo!(),
    //         CSSDisplay::Inline => todo!(),
    //         CSSDisplay::Flex => {
    //             match self.style.flex_direction {
    //                 CSSFlexDirection::Row => {
    //                     let width = children.iter().map(|child| child.x.clone()).reduce(|acc, el| acc + el).unwrap();
                        
    //                     let height = children.iter().map(|child| child.y.clone()).reduce(|mut acc, el| {
    //                         acc.or_max(el.get_max());
    //                         acc.and_min(el.get_min());
    //                         acc
    //                     }).unwrap();
        
    //                     Vec2::new(width, height)
    //                 },
    //                 CSSFlexDirection::Column => {
    //                     let width = children.iter().map(|child| child.x.clone()).reduce(|mut acc, el| {
    //                         acc.or_max(el.get_max());
    //                         acc.and_min(el.get_min());
    //                         acc
    //                     }).unwrap();
        
    //                     let height = children.iter().map(|child| child.y.clone()).reduce(|acc, el| acc + el).unwrap();
        
    //                     Vec2::new(width, height)
    //                 },
    //                 CSSFlexDirection::RowReverse => {
    //                     todo!()
    //                 },
    //                 CSSFlexDirection::ColumnReverse => {
    //                     todo!()
    //                 }
    //             }
    //         }
    //     }
    // }

    fn get_style(&self) -> Ref<'_, Style> {
        Ref::map(self.0.as_ref().borrow(), |borrow| &borrow.style)
    }
}

impl LayoutBox for Div {
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

    fn get_painter(&self, content_box: Rect, context: LayoutContext) -> PainterTree {
        todo!()
    }
}

impl BlockLayoutBox for Div {
    
}

impl Div {
    // fn flex_row(&self, context: &Context, size: Vec2) -> Vec<PainterTree> {
    //     let width = self.determine_width(context);

    //     let child_context = Context {
    //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(self.determine_width(context), 0))
    //     };

    //     // 1. Calculate desired sizes 

    //     // child, minimum width, curr = maximum_width, height
    //     let mut min_curr_width_height: Vec<(Rc<RefCell<dyn Element>>, i32, i32, i32)> = self.children.iter().map(|child| {
    //         let child_b = child.borrow();
    //         let bb = child_b.get_style();

    //         let size = bb.get_min_max_margin_area(&child_b, &child_context);
    //         let height = bb.get_height_from_width(&child_b, size.x.get_max().unwrap(), &child_context);

    //         (Rc::clone(child), size.x.get_min().unwrap(), size.x.get_max().unwrap(), height)
    //     }).collect();

    //     let desired_size: i32 = min_curr_width_height.iter().map(|(_, _, curr, _)| curr).sum();
    //     let error = width - desired_size;


    //     // 2. Distribute error
    //     if error > 0 {
    //         // Grow all elements by same ammout

    //         // TODO: Exact correction is float
    //         let correction = error / min_curr_width_height.len() as i32;
    //         min_curr_width_height.iter_mut().for_each(|(child, _, curr, height)| {
    //             *curr += correction;
    //             let child = child.borrow();
    //             *height = child.get_style().get_height_from_width(&child, *curr, &child_context);
    //         })

    //     } else if error < 0 {
    //         loop {
                
    //             // Error to distribute
    //             let desired_size: i32 = min_curr_width_height.iter().map(|(_, _, curr, _)| curr).sum();
    //             let error = width - desired_size;

    //             // Space distributed
    //             if error >= 0 {
    //                 break
    //             }
                
    //             // how many can adjust
    //             let unfrozen = min_curr_width_height.iter().filter(|(_, min, curr, _)| min != curr).count() as i32;

    //             // no one can be adjusted
    //             if unfrozen == 0 {
    //                 break
    //             }

    //             // positive correction
    //             let correction = -error / unfrozen;

    //             min_curr_width_height.iter_mut().filter(|(child, min, curr, height)| min != curr)
    //             .for_each(|(child, min, curr, height)| {
    //                 if *curr - correction >= *min {
    //                     *curr -= correction;
    //                 } else {
    //                     *curr = *min
    //                 }

    //                 let child = child.borrow();
    //                 *height = child.get_style().get_height_from_width(&child, *curr, &child_context);
    //             })
    //         }
    //     }

    //     let height = *min_curr_width_height.iter().map(|(_, _, _, curr)| curr).max().unwrap();
    //     let real_context = Context {
    //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(width, height))
    //     };
    //     let mut offset = Vec2::new(0, 0);

    //     // Get Painters
    //     min_curr_width_height.into_iter().map(|(child, _, curr, height)| {
    //         let child_b = child.borrow();
    //         let bb = child_b.get_style();

    //         let margin_box = Rect::new_vec(context.containing_block.position + offset, Vec2::new(curr, height));
    //         let content_box = margin_box - bb.get_total_computed_boudning(context);

    //         let painter = bb.get_painter(Rc::clone(&child), content_box, real_context.clone());
    //         offset.x += curr;
    //         painter
    //     }).collect()
    // }

    // fn flex_column(&self, context: &Context, size: Vec2) -> Vec<PainterTree> {
    //     let width = self.determine_width(context);

    //     println!("COLUMN: Width: {}", width);

    //     let height_range = self.determine_height_range(context);
        
    //     let child_context = Context {
    //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(width, 0))
    //     };
    //     // 1. Calculate desired sizes 

    //     // child, minimum height, curr = maximum_height, width
    //     let mut min_curr_height_width: Vec<(Rc<RefCell<dyn Element>>, i32, i32, i32)> = self.children.iter().map(|child| {
    //         let child_b = child.borrow();
    //         let bb = child_b.get_style();

    //         let min_max = bb.get_min_max_margin_area(&child_b, &child_context);
    //         let child_width = bb.get_width_from_height(&child_b, min_max.y.get_max().unwrap(), &child_context);

    //         (
    //             Rc::clone(child), 
    //             min_max.y.get_min().unwrap(), 
    //             min_max.y.get_max().unwrap().max(bb.get_height_from_width(&child_b, width, context)), 
    //             child_width
    //         )
    //     }).collect();

    //     // for (_, min, curr, width) in &min_curr_height_width {
    //     //     println!("COLUMN: Min: {}, Curr: {}, Width: {}", min, curr, width)
    //     // }

    //     let desired_size: i32 = min_curr_height_width.iter().map(|(_, _, curr, _)| curr).sum();
    //     let height = height_range.constrain(desired_size);
    //     let error = height - desired_size;

    //     // 2. Distribute error only if to short
    //     if error < 0 {
    //         loop {
                
    //             // Error to distribute
    //             let desired_size: i32 = min_curr_height_width.iter().map(|(_, _, curr, _)| curr).sum();
    //             let error = height - desired_size;

    //             // Space distributed
    //             if error >= 0 {
    //                 break
    //             }
                
    //             // how many can adjust
    //             let unfrozen = min_curr_height_width.iter().filter(|(_, min, curr, _)| min != curr).count() as i32;

    //             // no one can be adjusted
    //             if unfrozen == 0 {
    //                 break
    //             }

    //             // positive correction
    //             let correction = -error / unfrozen;

    //             min_curr_height_width.iter_mut().filter(|(child, min, curr, width)| min != curr).for_each(|(child, min, curr, width)| {
    //                 if *curr - correction >= *min {
    //                     *curr -= correction;
    //                 } else {
    //                     *curr = *min
    //                 }

    //                 let child = child.borrow();
    //                 *width = child.get_style().get_width_from_height(&child, *curr, &child_context);
    //             })
    //         }
    //     }

    //     let height = *min_curr_height_width.iter().map(|(_, _, curr, _)| curr).max().unwrap();
    //     let real_context = Context {
    //         containing_block: Rect::new_vec(context.containing_block.position, Vec2::new(width, height))
    //     };

    //     let mut offset = Vec2::new(0, 0);

    //     min_curr_height_width.into_iter().map(|(child, _, curr, _)| {
    //         let child_b = child.borrow();
    //         let bb = child_b.get_style();
            
    //         let margin_box = Rect::new_vec(context.containing_block.position + offset, Vec2::new(width, curr));
    //         let content_box = margin_box - bb.get_total_computed_boudning(context);

    //         let painter = bb.get_painter(Rc::clone(&child), content_box, real_context.clone());

    //         offset.y += curr;
    //         painter
    //     }).collect()
    // }
}

impl Div {
    fn determine_width(&self, context: &LayoutContext) -> i32 {
        let style = self.get_style();

        // Determine the available main and cross space for the flex items. For
        // each dimension, if that dimension of the flex container’s content box
        // is a definite size, use that; if that dimension of the flex container
        // is being sized under a min or max-content constraint, the available
        // space in that dimension is that constraint; otherwise, subtract the
        // flex container’s margin, border, and padding from the space available
        // to the flex container in that dimension and use that value.

        // 1. is definite
        if let Some(value) = style.width.solve(context) {
            return value
        }

        // 2. is under min/max constraint
        let min = style.min_width.solve(context);
        let max = style.max_width.solve(context);

        if min.is_some() || max.is_some() {
            return IndefRange::new_min_priority(min, max).constrain(context.containing_block.size.x)
        }
        
        // 3. margin, border and padding
        context.containing_block.size.x - style.get_total_bounding_width(context)
    }

    fn determine_height_range(&self, context: &LayoutContext) -> IndefRange {
        let style = self.get_style();

        // 1. is definite
        if let Some(value) = style.height.solve(context) {
            return IndefRange::new_definite(value)
        }

        // 2. is under min/max constraint
        let min = style.min_height.solve(context);
        let max = style.max_height.solve(context);

        IndefRange::new_option(min, max)
    }
}