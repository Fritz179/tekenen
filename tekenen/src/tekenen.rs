pub type Pixel = [u8; 4];
pub type Pixels = Vec<u8>;

use super::font::*;

#[allow(dead_code)]
pub mod colors {
    use super::Pixel;

    pub const WHITE: Pixel = [255, 255, 255, 255];
    pub const RED: Pixel = [255, 0, 0, 255];
    pub const GRAY: Pixel = [51, 51, 51, 255];
    pub const BLACK: Pixel = [0, 0, 0, 255];
}

pub struct Tekenen {
    pixels: Pixels,
    width: usize,
    height: usize,
}

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
}

// Drawing implementation
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

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Pixel) {
        for x in x..x + w {
            for y in y..y + h {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn circle(&mut self, x: i32, y: i32, r: i32, color: Pixel) {
        for xx in x-r..x+r {
            for yy in y-r..y+r {
                if (xx - x) * (xx - x) + (yy - y) * (yy - y) < r * r {
                    self.set_pixel(xx, yy, color);
                }
            }
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
        for x in x1..x2 {
            self.set_pixel(x, y, color);
            acc += ratio;

            while acc > 0.5 {
                y += 1;
                acc -= 1.0;
                self.set_pixel(x, y, color);
            }

            while acc < 0.5 {
                y -= 1;
                acc += 1.0;
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn background(&mut self, color: Pixel) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(x as i32, y as i32, color);
            }
        }
    }

    pub fn draw_image(&mut self, x: i32, y: i32, image: &Tekenen) {
        for xd in 0..image.width as i32 {
            for yd in 0..image.height as i32 {
                self.set_pixel(x + xd, y + yd, image.get_pixel(xd, yd).unwrap())
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
