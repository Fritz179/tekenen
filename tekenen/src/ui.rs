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

pub trait UIBox {
    fn draw(&mut self, view: ViewBox, tek: &mut Tekenen);
}

impl Tekenen {
    pub fn ui(&mut self, container: &mut Box<Container>) {
        let view = ViewBox::new(self);

        container.draw(view, self)
    }
}