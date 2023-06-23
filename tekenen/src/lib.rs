#![allow(dead_code)]

mod sdl;
mod tekenen;

pub use sdl::SDLPlatform as Platform;

use tekenen::Pixels;
pub use tekenen::{Tekenen, colors};

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

#[cfg(feature = "images")]
pub mod Image {
    use image::{io::Reader as ImageReader, GenericImageView};

    pub fn prelad_image(path: &str, to: &str) {
        let img = ImageReader::open(path).unwrap().decode().unwrap();

        let mut vec = vec![];

        for x in 0..img.width() {
            for y in 0..img.height() {
                let color = img.get_pixel(x, y);
                vec.push(color[0]);
                vec.push(color[1]);
                vec.push(color[2]);
                vec.push(color[3]);
            }
        };

        let out_dir = std::env::var_os("OUT_DIR").unwrap();

        let dest_path = std::path::Path::new(&out_dir).join("hello.rs");

        std::fs::write(
            &dest_path,
            "pub fn message() -> &'static str {
                \"Hello, World!\"
            }
            "
        ).unwrap();

        println!("cargo:rerun-if-changed={path}");
    }

}

#[derive(Debug)]
pub enum Keycode {
    Temp,
}

#[derive(Debug)]
pub struct Keymod {
    pub shift: bool,
    pub ctrl: bool,
    pub caps: bool,
}

#[derive(Debug)]
pub enum Event {
    KeyDown {
        repeat: bool,
        char: Option<char>,
        keycode: Keycode,
        keymod: Keymod,
    },
    Quit,
}

pub enum IntervalDecision {
    Repeat,
    Stop
}

pub trait PlatformTrait {
    fn new(width: u32, height: u32) -> Result<Self, PlatformError>
    where
        Self: Sized;
    fn display_pixels(&mut self, pixels: &tekenen::Pixels);
    fn read_events(&mut self) -> Option<Event>;
    fn set_interval(callback: impl FnMut() -> IntervalDecision + 'static, fps: u32);
    fn get_remaining_time() -> Duration;
}

use std::{error::Error, fmt, time::Duration};

#[derive(Debug)]
pub enum PlatformError {
    Init(String)
}

impl Error for PlatformError {}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}