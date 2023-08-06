mod sdl;
pub use sdl::SDLPlatform as Platform;

use crate::{Tekenen, tekenen};

#[derive(Debug)]
pub enum Keycode {
    ArrowUp,
    ArrowLeft,
    ArrowRight,
    ArrowDown,
    Enter,
    Escape,
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
        keycode: Option<Keycode>,
        keymod: Keymod,
    },
    MouseDown {
        x: i32,
        y: i32,
    },
    MouseUp {
        x: i32,
        y: i32,
    },
    MouseMove {
        x: i32,
        y: i32,
    },
    Quit,
}

pub enum IntervalDecision {
    Repeat,
    Stop
}

mod time_manager;

#[cfg(feature = "image")]
#[derive(Debug)]
#[cfg(feature = "image")]
pub enum ImageLoadingError {
    IOError(std::io::Error),
    ImageError(image::ImageError),
    MissingAssetError
}

pub trait PlatformTrait {
    fn new(width: u32, height: u32) -> Result<Self, PlatformError>
    where
        Self: Sized;
    fn display_pixels(&mut self, pixels: &tekenen::Pixels);
    fn read_events(&mut self) -> Option<Event>;
    fn set_interval(callback: impl FnMut() -> IntervalDecision + 'static, fps: u32);
    fn get_remaining_time() -> Duration;

    #[cfg(feature = "rust-embed")]
    fn set_assets<Asset: crate::rust_embed::DynRustEmbed + 'static>(asset: Asset);

    #[cfg(feature = "image")]
    fn load_image(path: &str) -> Result<Tekenen, ImageLoadingError>;

    #[cfg(feature = "image")]
    fn save_image(path: &str, image: &Tekenen) -> Result<(), image::ImageError> ;
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