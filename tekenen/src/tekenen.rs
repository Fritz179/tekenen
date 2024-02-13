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

use font::*;

use crate::{math::Vec2, platform::Event, shapes::{rect::Rect, Intersect, point::Point, circle::Circle, Shape}};

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

pub struct Font {
    height: i32,
}

impl Font {
    pub fn new(height: i32) -> Self {
        Self {
            height
        }
    }
}

pub enum OverflowBehavior {
    /// Draw everything
    Overflow,

    /// Draw only pixels inside the clip zone
    Hidden,

    /// Draw only shapes fully inside the clip zone
    Skip,

    /// Draw onyl shapes intersecting the clip zone
    MaybeFasterIDK
}

pub trait Draw {
    /// Draw any general shape
    fn shape<T: Shape>(&mut self, shape: T, color: Pixel);

    /// Blanket implementation for specific shapes
    /// Point
    
    // TODO: point
    // fn point(&mut self, x: i32, y: i32, color: Pixel) {
    //     self.shape(&Point::new(x, y), color)
    // }

    /// Rectangle
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Pixel) {
        self.shape(Rect::new(x, y, w, h), color)
    }

    fn rect_vec(&mut self, pos: Vec2, size: Vec2, color: Pixel) {
        self.shape(Rect::new_vec(pos, size), color)
    }

    /// Circle
    fn circle(&mut self, x: i32, y: i32, radius: i32, color: Pixel) {
        self.shape(Circle::new(x, y, radius), color)
    }

    fn circle_vec(&mut self, pos: Vec2, radius: i32, color: Pixel) {
        self.shape(Circle::vec(pos, radius), color);
    }

    fn background(&mut self, color: Pixel);

    fn text(&mut self, text: &str, x: i32, y: i32, font: Font) -> (i32, i32);

    fn text_vec(&mut self, text: &str, pos: Vec2, font: Font) -> (i32, i32) {
        self.text(text, pos.x, pos.y, font)
    }

    fn set_translation(&mut self, x: i32, y: i32) {
        self.set_translation_vec(Vec2::new(x, y))
    }

    fn set_translation_vec(&mut self, pos: Vec2);

    fn translate(&mut self, x: i32, y: i32) {
        self.translate_vec(Vec2::new(x, y))
    }

    fn translate_vec(&mut self, pos: Vec2);

    fn set_scale(&mut self, zoom: f32);

    fn scale(&mut self, zoom: f32);

    fn get_size(&self) -> Vec2;

    fn clip(&mut self, clip: Rect);

    fn reset_clip(&mut self);
}

pub struct Tekenen {
    /// The memory buffer holding the pixels
    pub pixels: Pixels,
    width: usize,
    height: usize,

    /// World coordinates
    /// Transformation
    translation: Vec2,
    zoom: f32,

    /// Screen coordinates
    /// clip object outside the clip area
    clip: Rect,
    overflow_behavior: OverflowBehavior,

    /// used for panning and zooming
    moving: bool,
}

// TODO: Load image
impl Tekenen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0; width * height * 4],
            width,
            height,
            translation: Vec2::default(),
            zoom: 1.0,
            clip: Rect::new(0, 0, width as i32, height as i32),
            overflow_behavior: OverflowBehavior::Overflow,
            moving: false
        }
    }

    pub fn from_pixels(width: usize, height: usize, pixels: Pixels) -> Self {
        Self {
            width,
            height,
            pixels,
            translation: Vec2::default(),
            zoom: 1.0,
            clip: Rect::new(0, 0, width as i32, height as i32),
            overflow_behavior: OverflowBehavior::Overflow,
            moving: false,
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

impl Tekenen {
    fn shape_impl<T: Shape>(&mut self, shape: T, color: Pixel) {
        for Vec2 {x, y} in shape.iter() {
            self.set_pixel(x, y, color);
        }
    }

    fn dyn_shape_impl(&mut self, shape: &dyn Shape, color: Pixel) {
        for Vec2 {x, y} in shape.iter() {
            self.set_pixel(x, y, color);
        }
    }

    pub fn handle_pan_and_zoom(&mut self, event: &Event) {
        match *event {
            Event::MouseDown { x, y } => {
                if self.clip.encloses_point(&Point::new(x ,y)) {
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

    /// offset => world offset to top left of screen
    /// scale => world pixel to screen pixel
    
    pub fn world_to_screen(&self, x: i32, y:i32) -> (i32, i32) {
        self.world_to_screen_vec(Vec2::new(x, y)).tuple()
    }

    pub fn world_to_screen_vec(&self, pos: Vec2) -> Vec2 {
        pos * self.zoom - self.translation
    }

    pub fn screen_to_world(&self, x: i32, y: i32) -> (i32, i32) {
        let x = ((x + self.translation.x) as f32 / self.zoom) as i32;
        let y = ((y + self.translation.y) as f32 / self.zoom) as i32;

        (x, y)
    }

    pub fn screen_to_world_vec(&self, pos: Vec2) -> Vec2 {
        (pos + self.translation) / self.zoom
    }
}

impl Draw for Tekenen {
    fn set_translation_vec(&mut self, pos: Vec2) {
        self.translation = pos
    }

    fn translate_vec(&mut self, pos: Vec2) {
        self.translation += pos
    }

    fn set_scale(&mut self, zoom: f32) {
        self.zoom = zoom
    }

    fn scale(&mut self, zoom: f32) {
        self.zoom += zoom
    }

    fn clip(&mut self, clip: Rect) {
        self.clip = clip
    }

    fn reset_clip(&mut self) {
        self.clip = Rect::new(0, 0, self.width as i32, self.height as i32)
    }

    fn shape<T: Shape>(&mut self, mut shape: T, color: Pixel) {
        shape.scale(self.zoom);
        shape.tranlsate(self.translation);

        match self.overflow_behavior {
            OverflowBehavior::Overflow => {
                self.shape_impl(shape, color)
            },
            OverflowBehavior::Skip => {
                if self.clip.encloses(shape.intersect_upcast()) {
                    self.shape_impl(shape, color)
                }
            },
            OverflowBehavior::Hidden => {
                let shape = shape.join_and(&self.clip);

                self.shape_impl(shape, color)

            },
            OverflowBehavior::MaybeFasterIDK => {
                if self.clip.intersect(shape.intersect_upcast()) {
                    self.shape_impl(shape, color)
                }
            }
        }
    }

    fn background(&mut self, color: Pixel) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(x as i32, y as i32, color);
            }
        }
    }

    fn text(&mut self, text: &str, x: i32, y: i32, font: Font) -> (i32, i32) {
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

    fn get_size(&self) -> Vec2 {
        Vec2::new(self.width as i32, self.height as i32)
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
}