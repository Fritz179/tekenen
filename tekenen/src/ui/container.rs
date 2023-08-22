use super::{BoundingBox, UIBox, ViewBox};

pub enum Direction {
    Horizonral,
    Vertical
}

pub enum Container {
    Single {
        bounding_box: BoundingBox,
        child: Box<dyn UIBox>
    },
    Directional {
        bounding_box: BoundingBox,
        direction: Direction,
        children: Vec<Box<dyn UIBox>>
    },
}

impl Container {
    pub fn vertical(children: Vec<Box<dyn UIBox>>) -> Box<Self> {
        Box::new(Self::Directional {
            bounding_box: BoundingBox::default(),
            direction: Direction::Vertical, 
            children
        })
    }

    pub fn horizontal(children: Vec<Box<dyn UIBox>>) -> Box<Self> {
        Box::new(Self::Directional {
            bounding_box: BoundingBox::default(),
            direction: Direction::Horizonral, 
            children
        })
    }
}

impl UIBox for Container {
    fn draw(&mut self, view: super::ViewBox, tek: &mut crate::Tekenen) {
        match self {
            Container::Single { bounding_box, child } => {
                child.draw(view, tek)
            },
            Container::Directional { bounding_box, direction: Direction::Horizonral, children } => {
                let size = children.len() as i32;
                let new_width = view.w / size;

                for i in 0..size {
                    let child = &mut children[i as usize];

                    let child_view = ViewBox {
                        x: view.x + new_width * i,
                        y: view.y,
                        w: new_width,
                        h: view.h
                    };

                    child.draw(child_view, tek)
                }
            },
            Container::Directional { bounding_box, direction: Direction::Vertical, children } => {
                let size = children.len() as i32;
                let new_height = view.h / size;

                for i in 0..size {
                    let child = &mut children[i as usize];

                    let child_view = ViewBox {
                        x: view.x,
                        y: view.y + new_height * i,
                        w: view.w,
                        h: new_height
                    };

                    child.draw(child_view, tek)
                }
            }
        }
    }
}
