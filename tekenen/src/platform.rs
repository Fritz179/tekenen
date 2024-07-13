#[cfg(feature = "native")]
mod sdl;
#[cfg(feature = "native")]
pub use sdl::SDLPlatform as Platform;

#[cfg(feature = "wasm")]
mod wasm;
#[cfg(feature = "wasm")]
pub use wasm::WASMPlatform as Platform;

#[cfg(feature = "image")]
use image::GenericImageView;

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
pub struct KeyDownEvent {
    pub repeat: bool,
    pub char: Option<char>,
    pub keycode: Option<Keycode>,
    pub keymod: Keymod,
}

impl KeyDownEvent {
    pub fn is_arrow(&self) -> bool {
        match self.keycode {
            Some(Keycode::ArrowUp) => true,
            Some(Keycode::ArrowLeft) => true,
            Some(Keycode::ArrowDown) => true,
            Some(Keycode::ArrowRight) => true,
            _ => false
        }
    } 
}

#[derive(Debug)]
pub enum Event {
    KeyDown (KeyDownEvent),
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
        xd: i32,
        yd: i32,
    },
    MouseWheel {
        direction: bool
    },
    Resize {
        w: i32,
        h: i32
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

// Fritz Preloaded Image Asset
const FPIA_MAGIC: [u8; 4] = [b'F', b'P', b'I', b'A'];

pub trait PlatformTrait {
    fn new(width: u32, height: u32) -> Result<Self, PlatformError>
    where
        Self: Sized;
    fn display_pixels(&mut self, pixels: &tekenen::Pixels);
    fn read_events(&mut self) -> Option<Event>;
    fn set_interval(callback: impl FnMut() -> IntervalDecision + 'static, fps: u32);
    fn get_remaining_time() -> Duration;

    #[cfg(feature = "image")]
    fn parse_image(data: &[u8]) -> Result<Tekenen, ImageLoadingError> {
        fn image_to_tekenen(img: image::DynamicImage) -> Tekenen {
            let mut pixels = vec![];

            for y in 0..img.height() {
                for x in 0..img.width() {
                    let color = img.get_pixel(x, y);
                    pixels.push(color[0]);
                    pixels.push(color[1]);
                    pixels.push(color[2]);
                    pixels.push(color[3]);
                }
            };
        
            let width = img.width() as usize;
            let height = img.height() as usize;
        
            Tekenen::from_pixels(width, height, pixels)
        }

        if data[0..4] == FPIA_MAGIC {
            let data = data;
            let (_magic, data) = data.split_at(4);

            assert!(data.len() >= 8);

            let (width, data) = data.split_at(4);
            let (height, data) = data.split_at(4);

            let width = u32::from_be_bytes(width.to_owned().try_into().unwrap()) as usize;
            let height = u32::from_be_bytes(height.to_owned().try_into().unwrap()) as usize;

            assert_eq!(data.len(), width * height * 4);

            Ok(Tekenen::from_pixels(width, height, data.to_owned()))
        } else {
            let img = image::load_from_memory(&data).map_err(ImageLoadingError::ImageError)?;
            Ok(image_to_tekenen(img))
        }
    }

    // #[cfg(feature = "image")]
    // fn save_image(path: &str, image: &Tekenen) -> Result<(), image::ImageError>;
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