pub type Pixel = [u8; 4];
pub type Pixels = Vec<u8>;

#[cfg(feature = "c64")]
mod font {
    mod font_c64;
    pub use font_c64::*;
}

#[cfg(not(feature = "c64"))]
mod font {
    mod font_default;
    pub use font_default::*;
}

use std::{rc::Rc, cell::RefCell};

use font::*;

use crate::{math::Vec2, platform::Event, shapes::{rect::Rect, Intersect, point::Point, circle::Circle, Shape, BitShaping, ComposedShape}};

#[allow(dead_code)]
pub mod colors {
    use super::Pixel;

    pub const RED: Pixel = [255, 0, 0, 255];
    pub const GREEN: Pixel = [0, 255, 0, 255];
    pub const BLUE: Pixel = [0, 0, 255, 255];

    pub const YELLOW: Pixel = [255, 255, 0, 255];
    pub const CYAN: Pixel = [0, 255, 255, 255];
    pub const MAGENTA: Pixel = [255, 0, 255, 255];

    pub const WHITE: Pixel = [255, 255, 255, 255];
    pub const SILVER: Pixel = [153, 153, 153, 255];
    pub const GRAY: Pixel = [51, 51, 51, 255];
    pub const BLACK: Pixel = [0, 0, 0, 255];
}

pub trait Draw {
    /// Draw any general shape
    fn shape(&mut self, shape: &dyn Shape, color: Pixel);

    /// Blanket implementation for specific shapes
    /// Rect
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Pixel) {
        self.shape(&Rect::new(x, y, w, h), color)
    }

    fn rect_at(&mut self, pos: Vec2, size: Vec2, color: Pixel) {
        self.shape(&Rect::vec(pos, size), color)
    }

    /// Circle
    fn circle(&mut self, x: i32, y: i32, r: i32, color: Pixel) {
        self.shape(&Circle::new(x, y, r), color)
    }

    fn circle_at(&mut self, pos: Vec2, radius: i32, color: Pixel) {
        self.shape(&Circle::vec(pos, radius), color)
    }

    fn background(&mut self, color: Pixel);
}

pub struct Tekenen {
    pub pixels: Pixels,
    width: usize,
    height: usize,
}

// TODO: Load image
impl Tekenen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0; width * height * 4],
            width,
            height,
        }
    }

    pub fn from_pixels(width: usize, height: usize, pixels: Pixels) -> Self {
        Self {
            width,
            height,
            pixels
        }
    }

    pub fn get_pixels(&self) -> &Pixels {
        &self.pixels
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Draw for Tekenen {
    fn shape(&mut self, shape: &dyn Shape, color: Pixel) {
        let shape = shape.dyn_clone();

        for Vec2 {x, y} in shape.iter() {
            self.set_pixel(x, y, color);
        }
    }

    fn background(&mut self, color: Pixel) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(x as i32, y as i32, color);
            }
        }
    }
}

impl Tekenen {
    pub fn pixel_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            Some((y * self.width as i32 + x) as usize)
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Pixel) {
        if let Some(index) = self.pixel_index(x, y) {
            // self.pixels.borrow_mut()[index] = color;
            self.pixels[index * 4 + 0] = color[0];
            self.pixels[index * 4 + 1] = color[1];
            self.pixels[index * 4 + 2] = color[2];
            self.pixels[index * 4 + 3] = color[3];
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Pixel> {
        if let Some(index) = self.pixel_index(x, y) {
            
            // TODO: return from slice?
            Some([
                self.pixels[index * 4 + 0],
                self.pixels[index * 4 + 1],
                self.pixels[index * 4 + 2],
                self.pixels[index * 4 + 3],
            ])
        } else {
            None
        }
    }

    pub fn line(&mut self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, color: Pixel) {
        if x1 > x2 {
            (x1, x2) = (x2, x1);
            (y1, y2) = (y2, y1);
        }

        let dx = x2 - x1;
        let dy = y2 - y1;

        let ratio = dy as f32 / dx as f32;

        let mut y = y1;
        let mut acc = 0.0;
        for x in x1..=x2 {
            self.set_pixel(x, y, color);
            acc += ratio;

            while acc > 0.5 {
                y += 1;
                acc -= 1.0;
                self.set_pixel(x, y, color);

                if y >= y2 {
                    break
                }
            }

            while acc < 0.5 {
                y -= 1;
                acc += 1.0;
                self.set_pixel(x, y, color);

                if y <= y2 {
                    break
                }
            }
        }
    }

    pub fn draw_scaled_image(&mut self, x: i32, y: i32, image: &Tekenen, scale: i32) {
        for xd in 0..image.width as i32 {
            for yd in 0..image.height as i32 {
                self.rect(x + xd * scale, y + yd * scale, scale, scale, image.get_pixel(xd, yd).unwrap())
            }
        }
    }

    pub fn draw_image(&mut self, x: i32, y: i32, image: &Tekenen) {
        for xd in 0..image.width as i32 {
            for yd in 0..image.height as i32 {
                let from = image.get_pixel(xd, yd).unwrap();

                // TODO: Proper color mixing
                if from[3] > 0 {
                    self.set_pixel(x + xd, y + yd, from)
                }
            }
        }
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32) -> (i32, i32) {
        const FONT_SCALE: i32 = 2;
        const FONT_SIZE: i32 = 8 * FONT_SCALE;

        let mut curr_x = 0;
        let mut curr_y = 0;

        for char in text.chars() {
            if curr_x >= 800 || char == '\n' {
                curr_x = 0;
                curr_y += FONT_SIZE;

                if char == '\n' {
                    continue;
                }
            }

            // skip whitespace
            if char == ' ' {
                curr_x += FONT_SIZE;
                continue;
            }

            // get data by finding offset in charset
            let data = FONT.get(char as usize - FIRST_CHAR as usize);

            let data = if let Some(data) = data {
                data
            } else {
                println!("Invalid char! {}", char);
                &FONT['?' as usize]
            };

            for (yd, line) in data.iter().enumerate() {
                let y = y + yd as i32 * FONT_SCALE + curr_y;

                for (xd, symbol) in line.iter().enumerate() {
                    let x = x + xd as i32 * FONT_SCALE + curr_x;

                    if *symbol == ' ' {
                        continue;
                    }

                    for xf in 0..FONT_SCALE {
                        for yf in 0..FONT_SCALE {
                            self.set_pixel(x + xf, y + yf, colors::WHITE);
                        }
                    }
                }
            }

            // increment for next character
            curr_x += FONT_SIZE;
        }

        (curr_x, curr_y)
    }

    pub fn draw_terminal(&mut self, buffer: &str, time: u64) {
        let (x, y) = self.draw_text(buffer, 0, 0);

        const BLINKING_TIME: u64 = 500;

        if time % BLINKING_TIME > BLINKING_TIME / 2 {
            self.rect(x, y, 16, 16, colors::WHITE)
        }
    }
}

pub enum OverflowBehavior {
    Overflow,
    Hidden,
    Skip
}

pub struct TransforView<T: Draw = Tekenen> {
    target: Rc<RefCell<T>>,
    screen: Rect,
    word_position: Vec2,
    // word_size = screen_size * zoom
    zoom: f32, 
    moving: bool,
    overflow_behavior: OverflowBehavior
}

impl<T: Draw> TransforView<T> {
    pub fn new(x: i32, y: i32, w: i32, h: i32, target: Rc<RefCell<T>>) -> Self {
        Self {
            target,
            screen: Rect::new(x, y, w, h),
            word_position: Vec2::default(),
            zoom: 1.0,
            moving: false,
            overflow_behavior: OverflowBehavior::Overflow
        }
    }
}

impl Draw for TransforView {
    fn shape(&mut self, shape: &dyn Shape, color: Pixel) {
        let mut shape = shape.dyn_clone();

        shape.transform(self.word_position + self.screen.position, self.zoom);

        match self.overflow_behavior {
            OverflowBehavior::Overflow => {
                self.target.borrow_mut().shape(&*shape, color)
            },
            OverflowBehavior::Skip => {
                todo!()
                // if self.screen.encloses(shape as &dyn Shape as &dyn Intersect) {
                //     self.target.borrow_mut().shape(shape, color)
                // }
            },
            OverflowBehavior::Hidden => {
                todo!()
                // // let new_shape = shape.join(&self.screen);
                // let new_shape = Box::new(shape);

                // self.target.borrow_mut().shape(*new_shape, color)
                // // self.target.borrow_mut().shape((&shape as &dyn BitShaping) & (&self.screen as &dyn BitShaping), color)
            }
        }
    }

    fn background(&mut self, color: Pixel) {
        self.target.borrow_mut().shape(&self.screen, color)
    }
}

impl TransforView {
    pub fn scale(&mut self, scale: f32) {
        self.zoom *= scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.zoom = scale
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.word_position.add(x, y)
    }

    pub fn set_translate(&mut self, x: i32, y: i32) {
        self.word_position.set(x, y)
    }

    pub fn reset(&mut self) {
        self.zoom = 1.0;
        self.word_position.set(0, 0)
    }

    pub fn set_overflow_behavior(&mut self, behavior: OverflowBehavior) {
        self.overflow_behavior = behavior
    }

    pub fn bounding_box(&self) -> Rect {
        self.screen.clone()
    }

    pub fn handle_pan_and_zoom(&mut self, event: &Event) {
        match *event {
            Event::MouseDown { x, y } => {
                if self.bounding_box().encloses_point(&Point::new(x ,y)) {
                    dbg!(self.bounding_box(), &self.word_position , x, y);

                    self.moving = true
                }
            },
            Event::MouseMove { xd, yd, .. } => {
                if self.moving {
                    self.translate(xd, yd)
                }
            },
            Event::MouseUp { x, y } => {
                self.moving = false
            },
            Event::MouseWheel { direction } => {
                self.zoom *= if direction { 0.99 } else { 1.01 }
            }
            _ => {}
        }
    }
}