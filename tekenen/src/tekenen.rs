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

use crate::{math::{Vec2, Zero}, platform::Event, shapes::{circle::Circle, line::Line, point::Point, rect::Rect, Intersect, Shape}};


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

    // fn set_translation(&self, x: i32, y: i32) {
    //     self.set_translation_vec(Vec2::new(x, y))
    // }

    // fn set_translation_vec(&self, pos: Vec2);

    // fn translate(&self, x: i32, y: i32) {
    //     self.translate_vec(Vec2::new(x, y))
    // }

    // fn translate_vec(&self, pos: Vec2);

    // fn set_scale(&self, zoom: f32);

    // fn scale(&self, zoom: f32);

    // fn get_size(&self) -> Vec2;

    // fn clip(&self, clip: Rect);

    // fn reset_clip(&self);

    fn draw_image(&self, x: i32, y: i32, image: &Surface);
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

    /// Our own width and height
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
        self.pixels.borrow().width() as i32
    }

    fn height(&self) -> i32 {
        self.pixels.borrow().height() as i32
    }

    fn shape(&self, shape: impl Shape) {
        shape.draw_yourself(self);
    }

    fn background(&self, color: Pixel) {
        let mut pixels = self.pixels.borrow_mut();
        
        for x in 0..pixels.width() {
            for y in 0..pixels.height() {
                pixels.set_pixel(x as i32, y as i32, color);
            }
        }
    }

    fn text(&self, text: &str, x: i32, y: i32, height: i32) -> Vec2 {
        let mut pixels = self.pixels.borrow_mut();

        let mut pos = Vec2::new(x, y);
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

        for x in 0..pixels.width() {
            for y in 0..pixels.height() {
                pixels.set_pixel(x as i32, y as i32, fill_color);
            }
        }
    }

    fn circle(&self, x: i32, y: i32, radius: i32) {
        // TODO: circle
        // todo!()
    }

    fn draw_image(&self, x: i32, y: i32, image: &Surface) {
        for xd in 0..image.width() as i32 {
            for yd in 0..image.height() as i32 {
                todo!()
                // let from = image.get_pixel(xd, yd).unwrap();

                // // TODO: Proper color mixing
                // if from[3] > 0 {
                //     self.set_pixel(x + xd, y + yd, from)
                // }
            }
        }
    }
}

impl SurfaceView {
    pub fn new(width: usize, height: usize, surface: SurfaceSource) -> Self {
        let surface: SurfaceDestination = match surface {
            SurfaceSource::Surface(surface) => SurfaceDrawer::from_surface(surface).into(),
            SurfaceSource::SurfaceView(surface) => surface.into(),
        };

        Self {
            surface: Rc::new(surface),
            width,
            height,
            translation: Vec2::zero(),
            zoom: 1.0,
            clip: Rect::new(0, 0, width as i32, height as i32),
            overflow_behavior: OverflowBehavior::Overflow,
            moving: false
        }
    }

    pub fn width(&self) -> i32 {
        self.width as i32
    }

    pub fn height(&self) -> i32 {
        self.height as i32
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

// impl SourcableSurface for SurfaceView {
//     fn as_slice(&self) -> &[Pixel] {
//         self.surface.as_slice()
//     }

//     fn width(&self) -> i32 {
//         self.surface.width()
//     }

//     fn height(&self) -> i32 {
//         self.surface.height()
//     }
// }

impl SurfaceView {
    // pub fn handle_pan_and_zoom(&self, event: &Event) {
    //     match *event {
    //         Event::MouseDown { x, y } => {
    //             if self.clip.encloses_point(&Point::new(x ,y)) {
    //                 self.moving = true
    //             }
    //         },
    //         Event::MouseMove { xd, yd, .. } => {
    //             if self.moving {
    //                 self.translate(xd, yd)
    //             }
    //         },
    //         Event::MouseUp { x, y } => {
    //             self.moving = false
    //         },
    //         Event::MouseWheel { direction } => {
    //             self.zoom *= if direction { 0.99 } else { 1.01 }
    //         }
    //         _ => {}
    //     }
    // }

    // / offset => world offset to top left of screen
    // / scale => world pixel to screen pixel
    
    // pub fn world_to_screen(&self, x: i32, y:i32) -> (i32, i32) {
    //     self.world_to_screen_vec(Vec2::new(x, y)).tuple()
    // }

    // pub fn world_to_screen_vec(&self, pos: Vec2) -> Vec2 {
    //     pos * self.zoom - self.translation
    // }

    // pub fn screen_to_world(&self, x: i32, y: i32) -> (i32, i32) {
    //     let x = ((x + self.translation.x) as f32 / self.zoom) as i32;
    //     let y = ((y + self.translation.y) as f32 / self.zoom) as i32;

    //     (x, y)
    // }

    // pub fn screen_to_world_vec(&self, pos: Vec2) -> Vec2 {
    //     (pos + self.translation) / self.zoom
    // }
}

impl SurfaceView {
    // fn set_translation_vec(&self, pos: Vec2) {
    //     self.translation = pos
    // }

    // fn translate_vec(&self, pos: Vec2) {
    //     self.translation += pos
    // }

    // fn set_scale(&self, zoom: f32) {
    //     self.zoom = zoom
    // }

    // fn scale(&self, zoom: f32) {
    //     self.zoom += zoom
    // }

    // fn clip(&self, clip: Rect) {
    //     self.clip = clip
    // }

    // fn reset_clip(&self) {
    //     self.clip = Rect::new(0, 0, self.width as i32, self.height as i32)
    // }

    // fn get_size(&self) -> Vec2 {
    //     Vec2::new(self.width as i32, self.height as i32)
    // }
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
        self.surface.text(text, x, y, height)
    }

    fn draw_image(&self, x: i32, y: i32, image: &Surface) {
        todo!()
    }

    fn shape(&self, mut shape: impl Shape) {
        // shape.scale(self.zoom);
        // shape.tranlsate(self.translation);

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