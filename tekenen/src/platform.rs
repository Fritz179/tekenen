#[cfg(all(feature = "native", not(target_family = "wasm")))]
mod sdl;
#[cfg(all(feature = "native", not(target_family = "wasm")))]
pub use sdl::SDLPlatform as Platform;

#[cfg(all(not(feature = "native"), not(target_family = "wasm")))]
mod mock;
#[cfg(all(not(feature = "native"), not(target_family = "wasm")))]
pub use mock::MockPlatform as Platform;

// Disable for wasm debugging
#[cfg(target_family = "wasm")]
mod wasm;
#[cfg(target_family = "wasm")]
pub use wasm::WASMPlatform as Platform;

#[cfg(feature = "image")]
use image::GenericImageView;

use crate::{math::Vec2, tekenen};

#[derive(Debug, Clone, Copy)]
pub enum Keycode {
    ArrowUp,
    ArrowLeft,
    ArrowRight,
    ArrowDown,
    Enter,
    Escape,
}

#[derive(Debug, Clone, Copy)]
pub struct Keymod {
    pub shift: bool,
    pub ctrl: bool,
    pub caps: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyDownEvent {
    pub repeat: bool,
    pub char: Option<char>,
    pub keycode: Option<Keycode>,
    pub keymod: Keymod,
}

impl KeyDownEvent {
    pub fn is_arrow(&self) -> bool {
        matches!(self.keycode, 
            Some(Keycode::ArrowUp) | 
            Some(Keycode::ArrowLeft) | 
            Some(Keycode::ArrowDown) | 
            Some(Keycode::ArrowRight)
        )
    } 
}

#[derive(Debug, Clone, Copy)]
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
        direction: bool,
        position: Vec2
    },
    Resize {
        w: i32,
        h: i32
    },
    Quit,
}

impl Event {
    pub fn position(&self) -> Option<Vec2> {
        match self {
            Event::MouseDown { x, y } => Some(Vec2::new(*x, *y)),
            Event::MouseUp { x, y } => Some(Vec2::new(*x, *y)),
            Event::MouseMove { x, y, .. } => Some(Vec2::new(*x, *y)),
            _ => None
        }
    }

    pub fn translate(&mut self, offset: Vec2) {
        match self {
            Event::MouseDown { x, y } => {
                *x += offset.x;
                *y += offset.y;
            },
            Event::MouseUp { x, y } => {
                *x += offset.x;
                *y += offset.y;
            },
            Event::MouseMove { x, y, .. } => {
                *x += offset.x;
                *y += offset.y;
            },
            _ => {}
        }
    } 
}

pub enum IntervalDecision {
    Repeat,
    Stop
}

mod time_manager;

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
    fn new(width: u32, height: u32) -> Result<Self, PlatformError> where Self: Sized;

    fn log(value: u32);
    
    fn display_surface(&mut self, surface: Ref<tekenen::Surface>);
    fn read_events(&mut self) -> Option<Event>;
    fn set_interval(callback: impl FnMut() -> IntervalDecision + 'static, fps: u32);
    fn get_remaining_time() -> Duration;

    #[cfg(feature = "image")]
    fn parse_image(data: &[u8]) -> Result<tekenen::Surface, ImageLoadingError> {
        use crate::tekenen::Surface;


        fn image_to_tekenen(img: image::DynamicImage) -> tekenen::Surface {
            let mut pixels = vec![];

            for y in 0..img.height() {
                for x in 0..img.width() {
                    let color = img.get_pixel(x, y);
                    pixels.push([color[0], color[1], color[2], color[3]]);
                }
            };
        
            let width = img.width() as usize;
            let height = img.height() as usize;
        
            Surface::from_pixels(width, height, pixels)
        }

        if data[0..4] == FPIA_MAGIC {
            let (_magic, data) = data.split_at(4);

            assert!(data.len() >= 8);

            let (width, data) = data.split_at(4);
            let (height, data) = data.split_at(4);

            let width = u32::from_be_bytes(width.to_owned().try_into().unwrap()) as usize;
            let height = u32::from_be_bytes(height.to_owned().try_into().unwrap()) as usize;

            unsafe {
                assert_eq!(data.len(), width * height * 4);

                let data: &[[u8; 4]] = std::slice::from_raw_parts(data.as_ptr() as *const [u8; 4], data.len() / 4);
                Ok(Surface::from_pixels(width, height, data.to_owned()))
            }
        } else {
            let img = image::load_from_memory(data).map_err(ImageLoadingError::ImageError)?;
            Ok(image_to_tekenen(img))
        }
    }

    // #[cfg(feature = "image")]
    // fn save_image(path: &str, image: &Tekenen) -> Result<(), image::ImageError>;
}

use std::{cell::Ref, error::Error, fmt, time::Duration};

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