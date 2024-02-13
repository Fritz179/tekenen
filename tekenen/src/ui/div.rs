use crate::Tekenen;

use super::{BoundingBox, Element};

pub enum Direction {
    Horizonral,
    Vertical
}

/// A div is a flexbox by default
/// A div with a single element is a flexbox with a single element
pub struct Div {
    bounding_box: BoundingBox,
    direction: Direction,
    children: Vec<Box<dyn Element>>
}

impl Div {
    pub fn new(child: Box<dyn Element>) -> Box<Self> {
        Box::new(Self {
            bounding_box: BoundingBox::default(),
            direction: Direction::Vertical,
            children: vec![child]
        })
    }

    pub fn new_vertical(children: Vec<Box<dyn Element>>) -> Box<Self> {
        Box::new(Self {
            bounding_box: BoundingBox::default(),
            direction: Direction::Vertical, 
            children
        })
    }

    pub fn new_horizontal(children: Vec<Box<dyn Element>>) -> Box<Self> {
        Box::new(Self {
            bounding_box: BoundingBox::default(),
            direction: Direction::Horizonral, 
            children
        })
    }
}

impl Element for Div {
    fn event(&mut self, event: crate::platform::Event) {
        
    }

    fn update(&mut self) {
        
    }

    fn draw(&mut self, target: &mut Tekenen) {
        // let event = Event::Resize { w: self.width() as i32, h: self.height() as i32 };


        // match self {
        //     Container::Single { bounding_box, child } => {
        //         child.draw(tv)
        //     },
        //     Container::Directional { bounding_box, direction: Direction::Horizonral, children } => {
        //         let children_len = children.len() as i32;
        //         let size = tv.get_size();
        //         let new_width = size.x / children_len;

        //         for i in 0..children_len {
        //             let child = &mut children[i as usize];

        //             let mut child_tv = TempTV::new(tv, new_width * i, 0, new_width, size.y);

        //             child.draw(&mut child_tv)
        //         }
        //     },
        //     Container::Directional { bounding_box, direction: Direction::Vertical, children } => {
        //         let children_len = children.len() as i32;
        //         let size = tv.get_size();
        //         let new_height = size.y / children_len;

        //         for i in 0..children_len {
        //             let child = &mut children[i as usize];

        //             let mut child_tv = TempTV::new(tv, 0, new_height * i, size.x, new_height);

        //             child.draw(&mut child_tv)
        //         }
        //     }
        // }
    }

    // fn get_box(&mut self, max: BoundingBox) -> &BoundingBox {
    //     match self {
    //         Container::Single { bounding_box, child } => {
    //             child.get_box(max)
    //         },
    //         Container::Directional { ref mut bounding_box, direction, children } => {
    //             let mut height = 0;
    //             let mut width = 0;

    //             let mut child_width = max.width.pixels();
    //             let mut child_height = max.height.pixels();

    //             if let Direction::Horizonral = direction {
    //                 child_width /= children.len() as i32
    //             } else {
    //                 child_height /= children.len() as i32
    //             }

    //             children.iter_mut().for_each(|child| {
    //                 let view = child.get_box(BoundingBox::new(child_width, child_height));

    //                 if let Direction::Horizonral = direction {
    //                     width += view.width.pixels();
    //                     if view.height.pixels() > height {
    //                         height = view.height.pixels()
    //                     }
    //                 } else {
    //                     height += view.height.pixels();
    //                     if view.width.pixels() > width {
    //                         width = view.width.pixels()
    //                     }
    //                 }
    //             });

    //             *bounding_box = BoundingBox::new(width, height);
    //             bounding_box
    //         }
    //     }
    // }
}