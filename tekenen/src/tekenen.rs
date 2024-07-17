pub type Pixel = [u8; 4];

#[derive(Debug)]
pub struct Surface {
    pub pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![[0, 0, 0, 0]; width * height],
            width,
            height,
        }
    }

    pub fn from_pixels(width: usize, height: usize, pixels: Vec<Pixel>) -> Self {
        assert!(pixels.len() == width * height, "Invalid pixel length, expected: {}, but got {}x{}", pixels.len(), width, height);

        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn as_slice(&self) ->&[Pixel] {
        &self.pixels
    }

    pub fn width(&self) -> i32 {
        self.width as i32
    }

    pub fn height(&self) -> i32 {
        self.height as i32
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Pixel) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return
        }

        let index = (y * self.width as i32 + x) as usize;
        self.pixels[index] = color;
    }
}

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

use std::{cell::{Cell, Ref, RefCell}, rc::Rc};

use enum_dispatch::enum_dispatch;
use font::*;

use crate::{math::{Transform, Vec2, Zero}, platform::Event, shapes::{circle::Circle, line::Line, point::Point, rect::Rect, Intersect, Shape}};


pub mod colors;

#[derive(Debug, Clone)]
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

#[enum_dispatch]
pub trait DrawableSurface {
    fn width(&self) -> i32;

    fn height(&self) -> i32;

    /// Draw any general shape
    fn shape(&self, shape: impl Shape);

    /// Blanket implementation for specific shapes
    /// Point
    
    // TODO: point
    // fn point(&self, x: i32, y: i32) {
    //     self.shape(Point::new(x, y))
    // }

    fn fill_color(&self, color: Pixel);

    /// Line
    fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32);

    /// Rectangle
    fn rect(&self, x: i32, y: i32, w: i32, h: i32);

    /// Circle
    fn circle(&self, x: i32, y: i32, radius: i32);

    fn background(&self, color: Pixel);

    fn text(&self, text: &str, x: i32, y: i32, height: i32) -> Vec2;

    fn draw_image(&self, x: i32, y: i32, image: &Surface) {
        self.draw_image_at(x, y, image.width(), image.height(), image)
    }

    fn draw_image_scaled(&self, x: i32, y: i32, scale: f32, image: &Surface) {
        let w = (image.width() as f32 * scale) as i32;
        let h = (image.height() as f32 * scale) as i32;

        self.draw_image_at(x, y, w, h, image)
    }

    fn draw_image_at(&self, x: i32, y: i32, w: i32, h: i32, image: &Surface);
}

#[derive(Debug, Clone)]
pub struct SurfaceDrawer {
    /// The memory buffer holding the pixels
    pixels: Rc<RefCell<Surface>>,

    fill_color: Cell<Pixel>,
}

#[derive(Debug, Clone)]
pub struct SurfaceView {
    /// The memory buffer holding the pixels
    surface: Rc<SurfaceDestination>,

    /// Screen coordinates
    screen: Cell<Rect>,

    /// World coordinates
    translation: Cell<Vec2>,
    scale: Cell<f32>,

    /// clip object outside the clip area
    overflow_behavior: OverflowBehavior,

    /// used for panning and scaleing
    moving: Cell<bool>,    
}

#[enum_dispatch(DrawableSurface)]
#[derive(Debug)]
enum SurfaceDestination {
    SurfaceDrawer,
    SurfaceView
}

#[enum_dispatch]
pub trait SourcableSurface {
 
}

#[enum_dispatch(SourcableSurface)]
pub enum SurfaceSource {
    Surface,
    SurfaceView
}

impl SurfaceDrawer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: Rc::new(RefCell::new(Surface::new(width, height))),
            fill_color: Cell::new(colors::WHITE),
        }
    }

    pub fn from_surface(surface: Surface) -> Self {
        Self {
            pixels: Rc::new(RefCell::new(surface)),
            fill_color: Cell::new(colors::WHITE),
        }
    }
}

impl SourcableSurface for SurfaceDrawer {
    // fn as_slice(&self) -> Ref<[Pixel]> {

    //     let r1 = self.pixels.borrow();
    //     let r2 = Ref::map(r1, |pixels| pixels.as_slice());
    //     r2
    // }

    // fn width(&self) -> i32 {
    //     self.pixels.borrow().width() as i32
    // }

    // fn height(&self) -> i32 {
    //     self.pixels.borrow().height() as i32
    // }
}

impl DrawableSurface for SurfaceDrawer {
    fn fill_color(&self, color:Pixel) {
        self.fill_color.set(color)
    }

    fn width(&self) -> i32 {
        self.pixels.borrow().width()
    }

    fn height(&self) -> i32 {
        self.pixels.borrow().height()
    }

    fn shape(&self, shape: impl Shape) {
        shape.draw_yourself(self);
    }

    fn background(&self, color: Pixel) {
        let mut pixels = self.pixels.borrow_mut();
        
        for x in 0..pixels.width() {
            for y in 0..pixels.height() {
                pixels.set_pixel(x, y, color);
            }
        }
    }

    fn text(&self, text: &str, x: i32, y: i32, height: i32) -> Vec2 {
        let mut pixels = self.pixels.borrow_mut();

        let pos = Vec2::new(x, y);
        let x = pos.x;
        let y = pos.y;

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
                            pixels.set_pixel(x + xf, y + yf, self.fill_color.get());
                        }
                    }
                }
            }

            // increment for next character
            curr_x += FONT_SIZE;
        }

        Vec2::new(curr_x, curr_y + FONT_SIZE)
    }

    fn line(&self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) {
        let mut pixels = self.pixels.borrow_mut();
        let fill_color = self.fill_color.get();

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
            pixels.set_pixel(x, y, fill_color);
            acc += ratio;

            while acc > 0.5 {
                y += 1;
                acc -= 1.0;
                pixels.set_pixel(x, y, fill_color);

                if y >= y2 {
                    break
                }
            }

            while acc < 0.5 {
                y -= 1;
                acc += 1.0;
                pixels.set_pixel(x, y, fill_color);

                if y <= y2 {
                    break
                }
            }
        }
    }

    fn rect(&self, x: i32, y: i32, w: i32, h: i32) {
        let mut pixels = self.pixels.borrow_mut();
        let fill_color = self.fill_color.get();

        for x in x..(x + w) {
            for y in y..(y + h) {
                pixels.set_pixel(x, y, fill_color);
            }
        }
    }

    fn circle(&self, xc: i32, yc: i32, r: i32) {
        let mut pixels = self.pixels.borrow_mut();
        let fill_color = self.fill_color.get();

        for x in (xc - r)..(xc + r) {
            for y in (yc - r)..(yc + r) {
                if (xc - x)*(xc - x) + (yc - y)*(yc - y) <= r * r {
                    pixels.set_pixel(x, y, fill_color);
                }
            }
        }
    }

    fn draw_image(&self,x:i32,y:i32,image: &Surface) {
        let mut destination = self.pixels.borrow_mut();
        let source = image.as_slice();

        for xd in 0..image.width() {
            for yd in 0..image.height() {
                let from = source[(yd * image.width() + xd) as usize];

                // TODO: Proper color mixing
                if from[3] > 0 {
                    destination.set_pixel(x + xd, y + yd, from)
                }
            }
        }
    }

    fn draw_image_at(&self, x: i32, y: i32, w: i32, h: i32, image: &Surface) {
        if w == image.width() && h == image.height() {
            self.draw_image(x, y, image);
            return
        }

        let mut destination = self.pixels.borrow_mut();
        let source = image.as_slice();

        for xd in x..(x + w) {
            for yd in y..(y + h) {
                let xs = (xd - x) * image.width() / w;
                let ys = (yd - y) * image.height() / h;

                let from = source[(ys * image.width() + xs) as usize];

                // TODO: Proper color mixing
                if from[3] > 0 {
                    destination.set_pixel(xd, yd, from)
                }
            }
        }
    }
}

impl SurfaceView {
    pub fn new(width: i32, height: i32, surface: SurfaceSource) -> Self {
        let surface: SurfaceDestination = match surface {
            SurfaceSource::Surface(surface) => SurfaceDrawer::from_surface(surface).into(),
            SurfaceSource::SurfaceView(surface) => surface.into(),
        };

        Self {
            surface: Rc::new(surface),
            screen: Cell::new(Rect::new(0, 0, width, height)),
            translation: Cell::new(Vec2::zero()),
            scale: Cell::new(1.0),
            overflow_behavior: OverflowBehavior::Overflow,
            moving: Cell::new(false)
        }
    }

    pub fn width(&self) -> i32 {
        self.screen.get().size.x
    }

    pub fn height(&self) -> i32 {
        self.screen.get().size.y
    }

    pub fn get_surface(&self) -> Ref<Surface> {
        self.surface.get_surface()
    }
}

impl SurfaceDestination {
    fn get_surface(&self) -> Ref<Surface> {
        match self {
            SurfaceDestination::SurfaceDrawer(drawer) => drawer.pixels.borrow(),
            SurfaceDestination::SurfaceView(view) => view.get_surface()
        }
    }
}

impl SurfaceView {
    pub fn clip(&self, clip: Rect) {
        self.screen.set(clip)
    }

    pub fn reset_clip(&self) {
        self.screen.set(Rect::new(0, 0, self.width(), self.height()))
    }
    
    pub fn handle_pan_and_zoom(&self, event: &Event) -> bool {
        match event {
            Event::MouseDown{ x, y } => {
                if self.screen.get().encloses_point(&Point::new(*x, *y)) {
                    self.moving.set(true);
                }
                true
            },
            Event::MouseUp{ .. } => {
                self.moving.set(false);
                true
            },
            Event::MouseMove{ xd, yd, .. } => {
                if self.moving.get() {
                    self.translate_screen(Vec2::new(*xd, *yd));
                    true
                } else {
                    false
                }
            }
            Event::MouseWheel{ direction, position } => {
                let value = if *direction { 1.2 } else { 1.0 / 1.2 };
                self.scale_screen(value, *position);
                true
            },
            _ => { false }
        }
    }
}

impl SurfaceView {
    // Set transformation
    pub fn set_translation(&self, translation: Vec2) {
        self.translation.set(translation);
    }

    pub fn set_scale(&self, scale: f32) {
        self.scale.set(scale);
    }

    pub fn set_transformation(&self, translation: Vec2, scale: f32) {
        self.translation.set(translation);
        self.scale.set(scale);
    }

    pub fn reset_transformation(&self) {
        self.translation.set(Vec2::zero());
        self.scale.set(1.0);
    }

    // Change transformation
    pub fn translate_screen(&self, translation: Vec2) {
        self.translation.set(self.translation.get() + translation);
    }

    pub fn translate(&self, translation: Vec2) {
        self.translation.set(self.translation.get() + translation * self.scale.get());
    }

    pub fn scale_screen(&self, scale: f32, mut from: Vec2) {
        from -= self.screen.get().position;
        self.translation.set(from + (self.translation.get() - from) * scale);
        self.scale.set(self.scale.get() * scale);
    }

    pub fn scale(&self, scale: f32) {
        self.scale.set(self.scale.get() * scale);
    }

    // Apply transformation
    pub fn world_to_screen(&self, target: &mut impl Transform) {
        target.scale(self.scale.get());
        target.translate(self.translation.get() + self.screen.get().position);
    }

    pub fn screen_to_world(&self, target: &mut impl Transform) {
        target.translate(-self.screen.get().position - self.translation.get());
        target.scale(1.0 / self.scale.get());
    }

    pub fn world_point_to_screen(&self, point: Vec2) -> Vec2 {
        point * self.scale.get() + self.translation.get() + self.screen.get().position
    }

    pub fn screen_point_to_world(&self, point: Vec2) -> Vec2 {
        (point - self.translation.get() - self.screen.get().position) / self.scale.get()
    }

    pub fn world_length_to_screen(&self, length: Vec2) -> Vec2 {
        length * self.scale.get()
    }

    pub fn screen_length_to_world(&self, length: f32) -> f32 {
        length / self.scale.get()
    }
}

impl DrawableSurface for SurfaceView {
    fn fill_color(&self, color: Pixel) {
        self.surface.fill_color(color)
    }

    fn width(&self) -> i32 {
        todo!()
    }

    fn height(&self) -> i32 {
        todo!()
    }

    fn background(&self, color: Pixel) {
        self.surface.background(color)
    }

    fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.shape(Line::new(x1, y1, x2, y2))
    }

    fn rect(&self, x: i32, y: i32, w: i32, h: i32) {
        self.shape(Rect::new(x, y, w, h))
    }

    fn circle(&self, x: i32, y: i32, radius: i32) {
        self.shape(Circle::new(x, y, radius))
    }

    fn text(&self, text: &str, x: i32, y: i32, height: i32) -> Vec2 {
        let pos = self.world_point_to_screen(Vec2::new(x, y));

        // TODO: could probably be done better, very low priority
        let height = self.world_length_to_screen(Vec2::new(0, height));

        self.surface.text(text, pos.x, pos.y, height.y)
    }

    fn draw_image_at(&self, x: i32, y: i32, w: i32, h: i32, image: &Surface) {
        let pos = self.world_point_to_screen(Vec2::new(x, y));
        let size = self.world_length_to_screen(Vec2::new(w, h));

        self.surface.draw_image_at(pos.x, pos.y, size.x, size.y, image)
    }

    fn shape(&self, mut shape: impl Shape) {
        self.world_to_screen(&mut shape);

        self.surface.shape(shape);
        // match self.overflow_behavior {
        //     OverflowBehavior::Overflow => {
        //         self.shape_impl(shape)
        //     },
        //     OverflowBehavior::Skip => {
        //         if self.clip.encloses(shape.intersect_upcast()) {
        //             self.shape_impl(shape)
        //         }
        //     },
        //     OverflowBehavior::Hidden => {
        //         let shape = shape.join_and(&self.clip);

        //         self.shape_impl(shape)

        //     },
        //     OverflowBehavior::MaybeFasterIDK => {
        //         if self.clip.intersect(shape.intersect_upcast()) {
        //             self.shape_impl(shape)
        //         }
        //     }
        // }
    }
}

impl SurfaceView {
    // pub fn pixel_index(&self, x: i32, y: i32) -> Option<usize> {
    //     if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
    //         None
    //     } else {
    //         Some((y * self.width as i32 + x) as usize)
    //     }
    // }

    // pub fn set_pixel(&self, x: i32, y: i32) {
    //     if let Some(index) = self.pixel_index(x, y) {
    //         // self.pixels.borrow_mut()[index] = color;
    //         self.pixels[index * 4] = color[0];
    //         self.pixels[index * 4 + 1] = color[1];
    //         self.pixels[index * 4 + 2] = color[2];
    //         self.pixels[index * 4 + 3] = color[3];
    //     }
    // }

    // pub fn get_pixel(&self, x: i32, y: i32) -> Option<Pixel> {
    //     self.pixel_index(x, y).map(|index| [
    //             self.pixels[index * 4],
    //             self.pixels[index * 4 + 1],
    //             self.pixels[index * 4 + 2],
    //             self.pixels[index * 4 + 3],
    //         ])
    // }
}