use sdl2;

use std::cell::RefCell;
use std::time::{Duration, Instant};

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use sdl2::keyboard;

use crate::tekenen::Pixels;
use super::{PlatformTrait, PlatformError, Event, Keycode, Keymod, IntervalDecision, time_manager::{TimeAction, TimeManager}};
#[cfg(feature = "image")]
use super::ImageLoadingError;
#[cfg(feature = "image")]
use image::GenericImageView;

#[cfg(feature = "rust-embed")]
use crate::rust_embed::DynRustEmbed;

// Fritz Preloaded Image Asset
const FPIA_MAGIC: [u8; 4] = ['F' as u8, 'P' as u8, 'I' as u8, 'A' as u8];

thread_local! {
    #[cfg(feature = "rust-embed")]
    static EMBEDDED_ASSETS: RefCell<Option<Box<dyn DynRustEmbed>>> = RefCell::new(None);
}

pub struct SDLPlatform {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    start: Instant,
    last_update: Instant,
    active: bool,
}

use crate::Tekenen;

impl PlatformTrait for SDLPlatform {
    fn new(width: u32, height: u32) -> Result<SDLPlatform, PlatformError> {
        let sdl_context = sdl2::init().map_err(|_| PlatformError::Init("Cannot init sdl context".to_owned()))?;

        let video_subsystem = sdl_context.video().map_err(|_| PlatformError::Init("Cannot init video".to_owned()))?;

        let window = video_subsystem
            .window("Salve!", width, height)
            .position_centered()
            .build()
            .map_err(|_| PlatformError::Init("Cannot create window".to_owned()))?;

        let canvas = window.into_canvas().build().map_err(|_| PlatformError::Init("Cannot create canvas".to_owned()))?;
        let event_pump = sdl_context.event_pump().map_err(|_| PlatformError::Init("Cannot create evet_pump".to_owned()))?;

        let io_manger = SDLPlatform {
            canvas,
            event_pump,
            start: Instant::now(),
            last_update: Instant::now(),
            active: true,
        };

        Ok(io_manger)
    }

    fn display_pixels(&mut self, pixels: &Pixels) {
        let (width, height) = self.canvas.output_size().expect("Cannot get canvas size");

        assert!(
            width * height * 4 == pixels.len() as u32,
            "Cannot render pixels!, Expected: {}, found: {}",
            width * height * 4,
            pixels.len()
        );

        let creator = self.canvas.texture_creator();
        let sprite = Rect::new(0, 0, width, height);

        let mut texture = creator
            .create_texture(
                sdl2::pixels::PixelFormatEnum::RGBA32,
                sdl2::render::TextureAccess::Target,
                width,
                height,
            )
            .unwrap();

        texture.update(sprite, pixels, (800 * 4) as usize).unwrap();

        let sprite = Rect::new(0, 0, width, height);
        self.canvas
            .copy(&texture, sprite, sprite)
            .expect("Cannot copy texture to canvas.");

        self.canvas.present();
    }

    fn read_events(&mut self) -> Option<Event> {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    self.active = false;
                    return Some(Event::Quit);
                }
                sdl2::event::Event::KeyDown {
                    keymod,
                    keycode: Some(keycode),
                    repeat,
                    ..
                } => {
                    // println!("{:?}", keycode);

                    let shift_mod: bool = keymod.bits()
                        & (sdl2::keyboard::Mod::LSHIFTMOD.bits()
                            | sdl2::keyboard::Mod::RSHIFTMOD.bits())
                        != 0;
                    let ctrl_mod: bool = keymod.bits()
                        & (sdl2::keyboard::Mod::LCTRLMOD.bits()
                            | sdl2::keyboard::Mod::RCTRLMOD.bits())
                        != 0;
                    let caps_mod: bool = keymod.bits() & sdl2::keyboard::Mod::CAPSMOD.bits() != 0;

                    let charcode = keycode as u32;
                    let mut char = None;

                    // Standard ascii code
                    if charcode >= ' ' as u32 && charcode <= '~' as u32 {
                        char = char::from_u32(charcode);
                    }

                    if shift_mod {
                        match keycode {
                            keyboard::Keycode::Minus => char = Some('_'),
                            keyboard::Keycode::Comma => char = Some(';'),
                            _ => {}
                        }
                    }

                    let keycode = match keycode {
                        keyboard::Keycode::Up => Some(Keycode::ArrowUp),
                        keyboard::Keycode::Left => Some(Keycode::ArrowLeft),
                        keyboard::Keycode::Right => Some(Keycode::ArrowRight),
                        keyboard::Keycode::Down => Some(Keycode::ArrowDown),
                        keyboard::Keycode::Return => Some(Keycode::Enter),
                        keyboard::Keycode::Escape => Some(Keycode::Escape),
                        // code => {
                        //     println!("{:?}", code);
                        //     None
                        // },
                        _ => None
                    };

                    return Some(Event::KeyDown {
                        repeat,
                        char,
                        keycode,
                        keymod: Keymod {
                            shift: shift_mod,
                            ctrl: ctrl_mod,
                            caps: caps_mod,
                        },
                    });
                },
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    return Some(Event::MouseDown { x, y })
                },
                sdl2::event::Event::MouseButtonUp { x, y, .. } => {
                    return Some(Event::MouseUp { x, y })
                },
                sdl2::event::Event::MouseMotion { x, y, .. } => {
                    return Some(Event::MouseMove { x, y })
                },
                _ => {
                    // println!("Unhandled event: {:?}", event);
                }
            }
        }

        None
    }

    fn set_interval(callback: impl FnMut() -> IntervalDecision + 'static, fps: u32) {
        let now = Instant::now();
        let interval = Duration::from_micros(1_000_000 / fps as u64);
        let fire_at = now + interval;

        TimeManager::add(TimeAction::Repeat {
            callback: Box::new(callback),
            fire_at,
            interval
        });

        TimeManager::spin();
    }

    fn get_remaining_time() -> Duration {
        TimeManager::get_remaining_time()
    }

    #[cfg(feature = "rust-embed")]
    fn set_assets<Asset: DynRustEmbed + 'static>(asset: Asset) {
        EMBEDDED_ASSETS.with(|e| {
            *e.borrow_mut() = Some(Box::new(asset))
        })
    }

    #[cfg(feature = "image")]
    fn load_image(path: &str) -> Result<Tekenen, ImageLoadingError> {
        println!("Loading image: {path}");

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

        #[cfg(not(feature = "rust-embed"))]
        {
            let path = std::path::Path::new(path);
            let img = image::io::Reader::open(path).or_else(|err| Err(ImageLoadingError::IOError(err)))?;
            let img = img.decode().or_else(|err| Err(ImageLoadingError::ImageError(err)))?;
            Ok(image_to_tekenen(img))
        }

        #[cfg(feature = "rust-embed")]
        EMBEDDED_ASSETS.with(|e| -> Result<Tekenen, ImageLoadingError> {
            match e.borrow().as_ref() {
                None => {
                    let path = std::path::Path::new(path);
                    let img = image::io::Reader::open(path).or_else(|err| Err(ImageLoadingError::IOError(err)))?;
                    let img = img.decode().or_else(|err| Err(ImageLoadingError::ImageError(err)))?;
                    Ok(image_to_tekenen(img))
                },
                Some(asset) => {
                    let source = asset.dyn_get(path).ok_or_else(|| ImageLoadingError::MissingAssetError)?;
    
                    
                    if source.data[0..4] == FPIA_MAGIC {
                        let data = source.data;
                        let (_magic, data) = data.split_at(4);
    
                        assert!(data.len() >= 8);
    
                        let (width, data) = data.split_at(4);
                        let (height, data) = data.split_at(4);
    
                        let width = u32::from_be_bytes(width.to_owned().try_into().unwrap()) as usize;
                        let height = u32::from_be_bytes(height.to_owned().try_into().unwrap()) as usize;
    
                        assert_eq!(data.len(), width * height * 4);
    
                        return Ok(Tekenen::from_pixels(width, height, data.to_owned()))
                    } else {
                        let img = image::load_from_memory(&source.data).or_else(|err| Err(ImageLoadingError::ImageError(err)))?;
                        Ok(image_to_tekenen(img))
                    }
                }
            }
        })
    }

    #[cfg(feature = "image")]
    fn save_image(path: &str, image: &Tekenen) -> Result<(), image::ImageError> {

        let buffer: &[u8] = image.get_pixels();   

    
        let path = std::path::Path::new(&path);

        image::save_buffer(&path, buffer, image.width() as u32, image.height() as u32, image::ColorType::Rgba8)?;

        println!("Saved image: {:?}", path);

        Ok(())
    }
}