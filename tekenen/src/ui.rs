use super::Tekenen;

pub mod container;
pub use container::Container;

pub mod slider;
pub use slider::Slider;

pub mod text;
pub use text::Text;

#[derive(Debug, Default)]
pub enum Unit {
    #[default]
    Auto,
    Pixels(i32),
    Percent(f32),
}

impl Unit {
    fn pixels(&self) -> i32 {
        match self {
            Unit::Auto => panic!("Auto no pixels"),
            Unit::Percent(_) => panic!("Percent no pixels"),
            Unit::Pixels(pixels) => *pixels
        }
    }
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

impl BoundingBox {
    fn new(width: i32, height: i32) -> Self {
        Self {
            margin: Sides::default(),
            border: Sides::default(),
            width: Unit::Pixels(width),
            height: Unit::Pixels(height)
        }
    }
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

pub trait UIBox {
    fn draw(&mut self, view: ViewBox, tek: &mut Tekenen);
    fn get_box(&mut self, max: BoundingBox) -> &BoundingBox;
    // fn get_children(&mut self) -> &[Box<dyn UIBox>];
}

// 1) Get size by passing down max allowed space for 100%
// 2) Draw according to calculated size, invalidate all if needed
// 3) React to key/mouse?

impl Tekenen {
    pub fn ui(&mut self, container: &mut Box<Container>) {
        let view = container.get_box(BoundingBox::new(self.width() as i32, self.height() as i32));
        let view = ViewBox { x: 0, y: 0, w: view.width.pixels(), h: view.height.pixels() };

        container.draw(view, self)
    }
}