use sdl2;

use std::time::{Duration, Instant};

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use sdl2::keyboard;

use crate::PlatformError;

use super::{PlatformTrait, Pixels, Event, Keycode, Keymod};

mod time_manager;
use time_manager::{TimeManager, TimeAction};
use crate::IntervalDecision;

pub struct SDLPlatform {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    start: Instant,
    last_update: Instant,
    active: bool,
}

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

                    if keycode == keyboard::Keycode::Return {
                        char = Some('\n')
                    }

                    if shift_mod {
                        match keycode {
                            keyboard::Keycode::Minus => char = Some('_'),
                            keyboard::Keycode::Comma => char = Some(';'),
                            _ => {}
                        }
                    }

                    return Some(Event::KeyDown {
                        repeat,
                        char,
                        keycode: Keycode::Temp,
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
}