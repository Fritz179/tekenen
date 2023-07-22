use super::Tekenen;

pub mod widgets;

#[derive(Debug, Default)]
pub enum Unit {
    #[default]
    Auto,
    Pixel(i32),
    Percent(f32),
}

#[derive(Debug, Default)]
pub struct Sides {
    up: Unit,
    right: Unit,
    down: Unit,
    left: Unit,
}

#[derive(Debug, Default)]
pub struct BoundingBox {
    margin: Sides,
    border: Sides,
    width: Unit,
    height: Unit
}

pub struct ViewBox {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl ViewBox {
    pub fn new(tekenen: &Tekenen) -> Self {
        Self {
            x: 0,
            y: 0,
            w: tekenen.width() as i32,
            h: tekenen.height() as i32
        }
    }
}

pub enum Direction {
    Horizonral,
    Vertical
}
pub enum Container<'c> {
    Single {
        bounding_box: BoundingBox,
        draw: Box<dyn FnMut(&ViewBox, &mut Tekenen) + 'c>,
    },
    Directional {
        bounding_box: BoundingBox,
        direction: Direction,
        children: Vec<Container<'c>>
    },
}

impl<'c> Container<'c> {
    pub fn new(draw: impl FnMut(&ViewBox, &mut Tekenen) + 'c) -> Container<'c> {
        Container::Single {
            bounding_box: BoundingBox::default(),
            draw: Box::new(draw),
        }
    }

    pub fn vertical(children: Vec<Container>) -> Container {
        Container::Directional {
            bounding_box: BoundingBox::default(),
            direction: Direction::Vertical, 
            children
        }
    }

    pub fn horiziontal(children: Vec<Container>) -> Container {
        Container::Directional {
            bounding_box: BoundingBox::default(),
            direction: Direction::Horizonral, 
            children
        }
    }
}

impl Tekenen {
    fn ui_impl(&mut self, view: ViewBox, container: &mut Container) {
        match container {
            Container::Single { bounding_box, ref mut draw } => {
                draw(&view, self);
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

                    self.ui_impl(child_view, child)
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

                    self.ui_impl(child_view, child)
                }
            }
        }
    }

    pub fn ui_boxed(&mut self, view: ViewBox, mut container: Container) {
        self.ui_impl(view, &mut container)
    }


    pub fn ui(&mut self, container: Container) {
        let view = ViewBox::new(self);

        self.ui_boxed(view, container)
    }
}