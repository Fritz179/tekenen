use std::{cell::RefCell, rc::Rc};

use crate::{math::{IndefRange, Vec2}, Draw, Tekenen};

use super::{BoundingBox, Element, SpaceContraint, super::{UISide, UISize}};

/// Flex-diretion
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    /// Horizontal layout
    /// Width is the max width
    /// Height is the heighest element
    /// Children calculated with parent_size. = {width, 0}
    Row,
    /// Vertical layout
    /// Width is the max width
    /// Height is the sum of all elements heights
    /// Children calculated with parent_size. = {width, 0}
    Column
}

// TODO: The others
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly
}

pub enum AlignItems {
    Start,
    End,
    Center,
    Stretch
}

/// A div is a flexbox
/// A div with a single element is a flexbox with a single element
#[derive(Debug)]
pub struct Div {
    bounding_box: BoundingBox,
    direction: Direction,
    children: Vec<Rc<RefCell<dyn Element>>>
}

/*
Percentage resolution

width        => containing block's width
height       => containing block's height (if not zero)
padding      => containing block's width
margin       => containing block's width
left / right => containing block's width
top / bottom => containing block's height
font-size    => parent block's font-size
line-height  => self element's font-size

*/
impl Div {
    pub fn new(child: Rc<RefCell<dyn Element>>) -> Rc<RefCell<Div>> {
        Rc::new(RefCell::new(Div {
            bounding_box: BoundingBox::default(),
            direction: Direction::Column, 
            children: vec![child]
        }))
        
    }

    pub fn new_vertical(children: Vec<Rc<RefCell<dyn Element>>>) -> Rc<RefCell<Div>> {
        Rc::new(RefCell::new(Div {
            bounding_box: BoundingBox::default(),
            direction: Direction::Column, 
            children
        }))
    }

    pub fn new_horizontal(children: Vec<Rc<RefCell<dyn Element>>>) -> Rc<RefCell<Div>> {
        Rc::new(RefCell::new(Div {
            bounding_box: BoundingBox::default(),
            direction: Direction::Row, 
            children
        }))
    }
}


impl Element for Div {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn draw(&self, target: &mut Tekenen, space: Vec2) -> Vec2 {
        // 1. Get this restriction
        let (width, height_range) = self.determine_size(space);
        let apparent_size = Vec2::new(width, 0);

        // get children desired size
        let desired: Vec<SpaceContraint> = self.children.iter().map(|child| child.borrow()
            .get_layout()).collect();

        // 2. Flex
        match self.direction {
            Direction::Row => {
                // 1. Calculate desired sizes 

                // min, curr
                let mut min_curr: Vec<(i32, i32)> = desired.iter().map(|constraint| {
                    (constraint.get_min_content(), constraint.get_max_content())
                }).collect();

                let desired_size = min_curr.iter().map(|(_, curr)| curr).sum::<i32>();
                let error = width - desired_size;

                // 2. Distribute error
                if error > 0 {
                    // Grow all elements by same ammout

                    // TODO: Exact correction is float
                    let correction = error / min_curr.iter().count() as i32;
                    min_curr.iter_mut().for_each(|(_, curr)| *curr += correction)

                } else if error < 0 {
                    loop {
                        
                        // Error to distribute
                        let desired_size: i32 = min_curr.iter().map(|(_, curr)| curr).sum();
                        let error = width - desired_size;

                        // Space distributed
                        if error >= 0 {
                            break
                        }
                        
                        // how many can adjust
                        let unfrozen = min_curr.iter().filter(|(min, curr)| min != curr).count() as i32;
    
                        // no one can be adjusted
                        if unfrozen == 0 {
                            break
                        }
    
                        // positive correction
                        let correction = -error / unfrozen;
    
                        min_curr.iter_mut().filter(|(min, curr)| min != curr).for_each(|(min, curr)| {
                            if *curr - correction >= *min {
                                *curr -= correction;
                            } else {
                                *curr = *min
                            }
                        })
                    }
                }

                let sizes: Vec<i32> = min_curr.iter().map(|(_, curr)| *curr).collect();
                let mut max_height = 0;

                // 3. Draw
                let mut total = 0;
                self.children.iter().zip(sizes).zip(desired).for_each(|((child, width), constrint)| {
                    let child_size = child.borrow_mut().draw(target, Vec2::new(width, space.y));
                    
                    if child_size.y > max_height {
                        max_height = child_size.y
                    }
                    
                    println!("child_size: {child_size:?}, width: {width:?}");
                    target.translate(width, 0);
                    total += width;
                });

                target.translate(-total, 0);

                if max_height > space.y {
                    max_height = space.y
                }

                // 4. Return size
                Vec2::new(space.x, max_height)
            },
            Direction::Column => {
                // 1. Calculate desired sizes 

                // min, curr
                let mut min_curr: Vec<(i32, i32)> = desired.iter().map(|constraint| {
                    (constraint.get_height(constraint.get_min_content(), apparent_size), constraint.get_height(constraint.get_max_content(), apparent_size))
                }).collect();

                let desired_size: i32 = min_curr.iter().map(|(_, curr)| curr).sum();
                let error = width - desired_size;

                // 2. Distribute error only if to short
                if error < 0 {
                    loop {
                        
                        // Error to distribute
                        let desired_size = min_curr.iter().map(|(_, curr)| curr).sum::<i32>();
                        let error = width - desired_size;

                        // Space distributed
                        if error >= 0 {
                            break
                        }
                        
                        // how many can adjust
                        let unfrozen = min_curr.iter().filter(|(min, curr)| min != curr).count() as i32;
    
                        // no one can be adjusted
                        if unfrozen == 0 {
                            break
                        }
    
                        // positive correction
                        let correction = -error / unfrozen;
    
                        min_curr.iter_mut().filter(|(min, curr)| min != curr).for_each(|(min, curr)| {
                            if *curr - correction >= *min {
                                *curr -= correction;
                            } else {
                                *curr = *min
                            }
                        })
                    }
                }

                let sizes: Vec<i32> = min_curr.iter().map(|(_, curr)| *curr).collect();

                // 3. Draw
                let mut total = 0;
                self.children.iter().zip(sizes).zip(desired).for_each(|((child, height), constrint)| {
                    let child_size = child.borrow_mut().draw(target, Vec2::new(constrint.get_width(height, space), space.y));
                    
                    // TODO: Should be height
                    target.translate(0, child_size.y);
                    total += child_size.y;
                });

                target.translate(0, -total);

                // 4. Return size
                Vec2::new(space.x, total)
            }
        }
    }

    fn get_layout(&self) -> SpaceContraint {    
        // 1. Get children restrictions
        let children_space = SpaceContraint::new_combined(
            self.children.iter().map(|child| child.borrow().get_layout()).collect(), 
            self.direction.clone(),
            (0, 10000) // TODO: ?
        );

        // 2. TODO: Add self restrictions (min / max)

        // TODO: Add self constraint
        children_space
    }
}

impl Div {
    fn constrain_dimension((start, end, size): (&UISide, &UISide, &UISize), parent_size: i32) -> i32 {
        // Determine the available main and cross space for the flex items. For
        // each dimension, if that dimension of the flex container’s content box
        // is a definite size, use that; if that dimension of the flex container
        // is being sized under a min or max-content constraint, the available
        // space in that dimension is that constraint; otherwise, subtract the
        // flex container’s margin, border, and padding from the space available
        // to the flex container in that dimension and use that value.

        // 1. is definite
        if let Some(value) = size.value.get_pixels(parent_size) {
            return value
        }


        // 2. is under min/max constraint
        let constraint = size.get_constraint(parent_size);

        if constraint.is_constrained() {
            return constraint.constrain(parent_size)
        }

        // 3. margin, border and padding
        return parent_size - start.total_size(parent_size) - end.total_size(parent_size)
    }

    fn constrain_dimension_range((start, end, size): (&UISide, &UISide, &UISize), parent_size: i32) -> IndefRange {
        // 1. height is definite
        if let Some(value) = size.value.get_pixels(parent_size) {
            return IndefRange::new_definite(value)
        }

        // 2. height is under min/max constraint
        size.get_constraint(parent_size)
    }
    

    // https://www.w3.org/TR/css-flexbox-1/#layout-algorithm
    fn determine_size(&self, parent_size: Vec2) -> (i32, IndefRange) {
        let bbox = &self.bounding_box;

        let width = Self::constrain_dimension((&bbox.left, &bbox.right, &bbox.width), parent_size.x);
        let height = Self::constrain_dimension_range((&bbox.up, &bbox.down, &bbox.height), parent_size.y);

        (width, height)
    }
}