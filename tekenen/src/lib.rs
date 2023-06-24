#![allow(dead_code)]

mod sdl;
mod tekenen;

pub use sdl::SDLPlatform as Platform;

use tekenen::Pixels;
pub use tekenen::{Tekenen, colors, ui};

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