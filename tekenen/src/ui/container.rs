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

    fn get_box(&mut self, max: BoundingBox) -> &BoundingBox {
        match self {
            Container::Single { bounding_box, child } => {
                child.get_box(max)
            },
            Container::Directional { ref mut bounding_box, direction, children } => {
                let mut height = 0;
                let mut width = 0;

                let mut child_width = max.width.pixels();
                let mut child_height = max.height.pixels();

                if let Direction::Horizonral = direction {
                    child_width /= children.len() as i32
                } else {
                    child_height /= children.len() as i32
                }

                children.iter_mut().for_each(|child| {
                    let view = child.get_box(BoundingBox::new(child_width, child_height));

                    if let Direction::Horizonral = direction {
                        width += view.width.pixels();
                        if view.height.pixels() > height {
                            height = view.height.pixels()
                        }
                    } else {
                        height += view.height.pixels();
                        if view.width.pixels() > width {
                            width = view.width.pixels()
                        }
                    }
                });

                *bounding_box = BoundingBox::new(width, height);
                bounding_box
            }
        }
    }
}
