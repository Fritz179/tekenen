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

use crate::{math::Vec2, platform::Event, shapes::{rect::Rect, Intersect, point::Point, circle::Circle}};

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
    fn rect(&mut self, rect: Rect, color: Pixel);
    fn circle(&mut self, circle: Circle, color: Pixel);

    fn rect_at(&mut self, pos: Vec2, size: Vec2, color: Pixel) {
        self.rect(Rect::vec(pos, size), color)
    }

    fn circle_at(&mut self, pos: Vec2, radius: i32, color: Pixel) {
        self.circle(Circle::vec(pos, radius), color)
    }

    fn rect_raw(&mut self, x: i32, y: i32, w: i32, h: i32, color: Pixel) {
        self.rect(Rect::new(x, y, w, h), color)
    }

    fn circle_raw(&mut self, x: i32, y: i32, r: i32, color: Pixel) {
        self.circle(Circle::new(x, y, r), color)
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
    fn rect(&mut self, rect: Rect, color: Pixel) {
        for x in rect.position.x..rect.position.x + rect.size.x {
            for y in rect.position.y..rect.position.y + rect.size.y {
                self.set_pixel(x, y, color);
            }
        }
    }

    fn circle(&mut self, circle: Circle, color: Pixel) {
        for xx in circle.position.x-circle.radius..circle.position.x+circle.radius {
            for yy in circle.position.y-circle.radius..circle.position.y+circle.radius {
                if (xx - circle.position.x) * (xx - circle.position.x) + (yy - circle.position.y) * (yy - circle.position.y) < circle.radius * circle.radius {
                    self.set_pixel(xx, yy, color);
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
                self.rect_raw(x + xd * scale, y + yd * scale, scale, scale, image.get_pixel(xd, yd).unwrap())
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
            self.rect_raw(x, y, 16, 16, colors::WHITE)
        }
    }
}

pub struct TransforView {
    target: Rc<RefCell<dyn Draw>>,
    screen_posizion: Vec2,
    screen_size: Vec2,
    word_position: Vec2,
    // word_size = screen_size * zoom
    zoom: f32, 
    moving: bool,
}

impl TransforView {
    pub fn new(x: i32, y: i32, w: i32, h: i32, target: Rc<RefCell<dyn Draw>>) -> Self {
        Self {
            target,
            screen_posizion: Vec2::new(x ,y),
            screen_size: Vec2::new(w, h),
            word_position: Vec2::default(),
            zoom: 1.0,
            moving: false,
        }
    }
}

impl Draw for TransforView {
    fn rect(&mut self, rect: Rect, color: Pixel) {
        let Rect { position: Vec2 {x, y}, size: Vec2 { x: w, y: h } } = rect;

        let x = (x as f32 * self.zoom) as i32 + self.word_position.x + self.screen_posizion.x;
        let y = (y as f32 * self.zoom) as i32 + self.word_position.y + self.screen_posizion.y;
        let w = w as f32 * self.zoom;
        let h = h as f32 * self.zoom;

        // print!("{x}, {y}, {w}, {h}");

        self.target.borrow_mut().rect(Rect::new(x as i32, y as i32, w as i32, h as i32), color)
    }

    fn circle(&mut self, circle: Circle, color: Pixel) {
        let Circle { position: Vec2 { x, y }, radius: r } = circle;

        let x = x as f32 * self.zoom + self.word_position.x as f32 + self.screen_posizion.x as f32;
        let y = y as f32 * self.zoom + self.word_position.y as f32 + self.screen_posizion.y as f32;
        let r = r as f32 * self.zoom;

        self.target.borrow_mut().circle(Circle::new(x as i32, y as i32, r as i32), color)
    }

    fn background(&mut self, color: Pixel) {
        let x = self.screen_posizion.x;
        let y = self.screen_posizion.y;

        let w = self.screen_size.x;
        let h = self.screen_size.y;

        self.target.borrow_mut().rect_raw(x, y, w as i32, h as i32, color)
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

    pub fn bounding_box(&self) -> Rect {
        Rect {
            position: Vec2 {
                x: self.word_position.x,
                y: self.word_position.y,
            },
            size: Vec2 {
                x: (self.screen_size.x as f32 * self.zoom) as i32,
                y: (self.screen_size.y as f32 * self.zoom) as i32,
            }
        }
    }

    pub fn handle_pan_and_zoom(&mut self, event: Event) {
        match event {
            Event::MouseDown { x, y } => {
                dbg!(self.bounding_box(), x, y);

                if self.bounding_box().encloses_point(&Point::new(x ,y)) {
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
            _ => {}
        }
    }
}