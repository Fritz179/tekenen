use sdl2;

use std::cell::Ref;
use std::time::{Duration, Instant};

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use sdl2::keyboard;

use crate::math::Vec2;
use crate::Surface;

use super::MouseButton;
use super::{PlatformTrait, PlatformError, Event, KeyDownEvent, Keycode, KeyModifiers, IntervalDecision, time_manager::{TimeAction, TimeManager}};

pub struct SDLPlatform {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    start: Instant,
    last_update: Instant,
    active: bool,
    mouse_position: Vec2,
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

        let io_manager = SDLPlatform {
            canvas,
            event_pump,
            start: Instant::now(),
            last_update: Instant::now(),
            active: true,
            mouse_position: Vec2::new(0, 0),
        };

        Ok(io_manager)
    }

    fn log(value: String) {
        println!("{value}");
    }

    fn display_surface(&mut self, pixels: Ref<Surface>) {
        let (width, height) = self.canvas.output_size().expect("Cannot get canvas size");

        assert!(
            width == pixels.width() as u32 && height == pixels.height() as u32,
            "Cannot render pixels!, Expected: {}x{}, found: {}x{}",
            width, height,
            pixels.width(), pixels.height()
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

        texture.update(sprite, pixels.as_slice().flatten(), (800 * 4) as usize).unwrap();

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

                    return Some(Event::KeyDown(KeyDownEvent{
                        repeat,
                        char,
                        keycode: keycode.into(),
                        modifiers: KeyModifiers {
                            shift: shift_mod,
                            ctrl: ctrl_mod,
                            caps: caps_mod,
                        },
                    }));
                },
                sdl2::event::Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    self.mouse_position = Vec2::new(x, y);
                    return Some(Event::MouseDown { x, y, key: mouse_btn.into() })
                },
                sdl2::event::Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    self.mouse_position = Vec2::new(x, y);
                    return Some(Event::MouseUp { x, y, key: mouse_btn.into() })
                },
                sdl2::event::Event::MouseMotion { x, y, xrel, yrel, .. } => {
                    self.mouse_position = Vec2::new(x, y);
                    return Some(Event::MouseMove { x, y, xd: xrel, yd: yrel })
                },
                sdl2::event::Event::MouseWheel { y, ..} => {
                    return Some(Event::MouseWheel { direction: y == 1, position: self.mouse_position })
                }
                _ => {
                    // println!("Unhandled event: {event:?}");
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

impl From<sdl2::mouse::MouseButton> for MouseButton {
    fn from(button: sdl2::mouse::MouseButton) -> Self {
        match button {
            sdl2::mouse::MouseButton::Left => MouseButton::Left,
            sdl2::mouse::MouseButton::Right => MouseButton::Right,
            sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
            _ => unimplemented!(),
        }
    }
}

impl From<sdl2::keyboard::Keycode> for Keycode {
    fn from(value: sdl2::keyboard::Keycode) -> Self {
        match value {
            sdl2::keyboard::Keycode::Backspace => Keycode::Backspace,
            sdl2::keyboard::Keycode::Tab => Keycode::Tab,
            sdl2::keyboard::Keycode::Return => Keycode::Enter,
            sdl2::keyboard::Keycode::Escape => Keycode::Escape,
            sdl2::keyboard::Keycode::Space => Keycode::Space,
            sdl2::keyboard::Keycode::Exclaim => Keycode::Exclaim,
            sdl2::keyboard::Keycode::Quotedbl => Keycode::Quotedbl,
            sdl2::keyboard::Keycode::Hash => Keycode::Hash,
            sdl2::keyboard::Keycode::Dollar => Keycode::Dollar,
            sdl2::keyboard::Keycode::Percent => Keycode::Percent,
            sdl2::keyboard::Keycode::Ampersand => Keycode::Ampersand,
            sdl2::keyboard::Keycode::Quote => Keycode::Quote,
            sdl2::keyboard::Keycode::LeftParen => Keycode::LeftParen,
            sdl2::keyboard::Keycode::RightParen => Keycode::RightParen,
            sdl2::keyboard::Keycode::Asterisk => Keycode::Asterisk,
            sdl2::keyboard::Keycode::Plus => Keycode::Plus,
            sdl2::keyboard::Keycode::Comma => Keycode::Comma,
            sdl2::keyboard::Keycode::Minus => Keycode::Minus,
            sdl2::keyboard::Keycode::Period => Keycode::Period,
            sdl2::keyboard::Keycode::Slash => Keycode::Slash,
            sdl2::keyboard::Keycode::Num0 => Keycode::Num0,
            sdl2::keyboard::Keycode::Num1 => Keycode::Num1,
            sdl2::keyboard::Keycode::Num2 => Keycode::Num2,
            sdl2::keyboard::Keycode::Num3 => Keycode::Num3,
            sdl2::keyboard::Keycode::Num4 => Keycode::Num4,
            sdl2::keyboard::Keycode::Num5 => Keycode::Num5,
            sdl2::keyboard::Keycode::Num6 => Keycode::Num6,
            sdl2::keyboard::Keycode::Num7 => Keycode::Num7,
            sdl2::keyboard::Keycode::Num8 => Keycode::Num8,
            sdl2::keyboard::Keycode::Num9 => Keycode::Num9,
            sdl2::keyboard::Keycode::Colon => Keycode::Colon,
            sdl2::keyboard::Keycode::Semicolon => Keycode::Semicolon,
            sdl2::keyboard::Keycode::Less => Keycode::Less,
            sdl2::keyboard::Keycode::Equals => Keycode::Equals,
            sdl2::keyboard::Keycode::Greater => Keycode::Greater,
            sdl2::keyboard::Keycode::Question => Keycode::Question,
            sdl2::keyboard::Keycode::At => Keycode::At,
            sdl2::keyboard::Keycode::LeftBracket => Keycode::LeftBracket,
            sdl2::keyboard::Keycode::Backslash => Keycode::Backslash,
            sdl2::keyboard::Keycode::RightBracket => Keycode::RightBracket,
            sdl2::keyboard::Keycode::Caret => Keycode::Caret,
            sdl2::keyboard::Keycode::Underscore => Keycode::Underscore,
            sdl2::keyboard::Keycode::Backquote => Keycode::Backquote,
            sdl2::keyboard::Keycode::A => Keycode::A,
            sdl2::keyboard::Keycode::B => Keycode::B,
            sdl2::keyboard::Keycode::C => Keycode::C,
            sdl2::keyboard::Keycode::D => Keycode::D,
            sdl2::keyboard::Keycode::E => Keycode::E,
            sdl2::keyboard::Keycode::F => Keycode::F,
            sdl2::keyboard::Keycode::G => Keycode::G,
            sdl2::keyboard::Keycode::H => Keycode::H,
            sdl2::keyboard::Keycode::I => Keycode::I,
            sdl2::keyboard::Keycode::J => Keycode::J,
            sdl2::keyboard::Keycode::K => Keycode::K,
            sdl2::keyboard::Keycode::L => Keycode::L,
            sdl2::keyboard::Keycode::M => Keycode::M,
            sdl2::keyboard::Keycode::N => Keycode::N,
            sdl2::keyboard::Keycode::O => Keycode::O,
            sdl2::keyboard::Keycode::P => Keycode::P,
            sdl2::keyboard::Keycode::Q => Keycode::Q,
            sdl2::keyboard::Keycode::R => Keycode::R,
            sdl2::keyboard::Keycode::S => Keycode::S,
            sdl2::keyboard::Keycode::T => Keycode::T,
            sdl2::keyboard::Keycode::U => Keycode::U,
            sdl2::keyboard::Keycode::V => Keycode::V,
            sdl2::keyboard::Keycode::W => Keycode::W,
            sdl2::keyboard::Keycode::X => Keycode::X,
            sdl2::keyboard::Keycode::Y => Keycode::Y,
            sdl2::keyboard::Keycode::Z => Keycode::Z,
            sdl2::keyboard::Keycode::Delete => Keycode::Delete,
            sdl2::keyboard::Keycode::CapsLock => Keycode::CapsLock,
            sdl2::keyboard::Keycode::F1 => Keycode::F1,
            sdl2::keyboard::Keycode::F2 => Keycode::F2,
            sdl2::keyboard::Keycode::F3 => Keycode::F3,
            sdl2::keyboard::Keycode::F4 => Keycode::F4,
            sdl2::keyboard::Keycode::F5 => Keycode::F5,
            sdl2::keyboard::Keycode::F6 => Keycode::F6,
            sdl2::keyboard::Keycode::F7 => Keycode::F7,
            sdl2::keyboard::Keycode::F8 => Keycode::F8,
            sdl2::keyboard::Keycode::F9 => Keycode::F9,
            sdl2::keyboard::Keycode::F10 => Keycode::F10,
            sdl2::keyboard::Keycode::F11 => Keycode::F11,
            sdl2::keyboard::Keycode::F12 => Keycode::F12,
            sdl2::keyboard::Keycode::PrintScreen => Keycode::PrintScreen,
            sdl2::keyboard::Keycode::ScrollLock => Keycode::ScrollLock,
            sdl2::keyboard::Keycode::Pause => Keycode::Pause,
            sdl2::keyboard::Keycode::Insert => Keycode::Insert,
            sdl2::keyboard::Keycode::Home => Keycode::Home,
            sdl2::keyboard::Keycode::PageUp => Keycode::PageUp,
            sdl2::keyboard::Keycode::End => Keycode::End,
            sdl2::keyboard::Keycode::PageDown => Keycode::PageDown,
            sdl2::keyboard::Keycode::Right => Keycode::ArrowRight,
            sdl2::keyboard::Keycode::Left => Keycode::ArrowLeft,
            sdl2::keyboard::Keycode::Down => Keycode::ArrowDown,
            sdl2::keyboard::Keycode::Up => Keycode::ArrowUp,
            sdl2::keyboard::Keycode::NumLockClear => Keycode::NumLockClear,
            sdl2::keyboard::Keycode::KpDivide => Keycode::KpDivide,
            sdl2::keyboard::Keycode::KpMultiply => Keycode::KpMultiply,
            sdl2::keyboard::Keycode::KpMinus => Keycode::KpMinus,
            sdl2::keyboard::Keycode::KpPlus => Keycode::KpPlus,
            sdl2::keyboard::Keycode::KpEnter => Keycode::KpEnter,
            sdl2::keyboard::Keycode::Kp1 => Keycode::Kp1,
            sdl2::keyboard::Keycode::Kp2 => Keycode::Kp2,
            sdl2::keyboard::Keycode::Kp3 => Keycode::Kp3,
            sdl2::keyboard::Keycode::Kp4 => Keycode::Kp4,
            sdl2::keyboard::Keycode::Kp5 => Keycode::Kp5,
            sdl2::keyboard::Keycode::Kp6 => Keycode::Kp6,
            sdl2::keyboard::Keycode::Kp7 => Keycode::Kp7,
            sdl2::keyboard::Keycode::Kp8 => Keycode::Kp8,
            sdl2::keyboard::Keycode::Kp9 => Keycode::Kp9,
            sdl2::keyboard::Keycode::Kp0 => Keycode::Kp0,
            sdl2::keyboard::Keycode::KpPeriod => Keycode::KpPeriod,
            sdl2::keyboard::Keycode::Application => Keycode::Application,
            sdl2::keyboard::Keycode::Power => Keycode::Power,
            sdl2::keyboard::Keycode::KpEquals => Keycode::KpEquals,
            sdl2::keyboard::Keycode::F13 => Keycode::F13,
            sdl2::keyboard::Keycode::F14 => Keycode::F14,
            sdl2::keyboard::Keycode::F15 => Keycode::F15,
            sdl2::keyboard::Keycode::F16 => Keycode::F16,
            sdl2::keyboard::Keycode::F17 => Keycode::F17,
            sdl2::keyboard::Keycode::F18 => Keycode::F18,
            sdl2::keyboard::Keycode::F19 => Keycode::F19,
            sdl2::keyboard::Keycode::F20 => Keycode::F20,
            sdl2::keyboard::Keycode::F21 => Keycode::F21,
            sdl2::keyboard::Keycode::F22 => Keycode::F22,
            sdl2::keyboard::Keycode::F23 => Keycode::F23,
            sdl2::keyboard::Keycode::F24 => Keycode::F24,
            sdl2::keyboard::Keycode::Execute => Keycode::Execute,
            sdl2::keyboard::Keycode::Help => Keycode::Help,
            sdl2::keyboard::Keycode::Menu => Keycode::Menu,
            sdl2::keyboard::Keycode::Select => Keycode::Select,
            sdl2::keyboard::Keycode::Stop => Keycode::Stop,
            sdl2::keyboard::Keycode::Again => Keycode::Again,
            sdl2::keyboard::Keycode::Undo => Keycode::Undo,
            sdl2::keyboard::Keycode::Cut => Keycode::Cut,
            sdl2::keyboard::Keycode::Copy => Keycode::Copy,
            sdl2::keyboard::Keycode::Paste => Keycode::Paste,
            sdl2::keyboard::Keycode::Find => Keycode::Find,
            sdl2::keyboard::Keycode::Mute => Keycode::Mute,
            sdl2::keyboard::Keycode::VolumeUp => Keycode::VolumeUp,
            sdl2::keyboard::Keycode::VolumeDown => Keycode::VolumeDown,
            sdl2::keyboard::Keycode::KpComma => Keycode::KpComma,
            sdl2::keyboard::Keycode::KpEqualsAS400 => Keycode::KpEqualsAS400,
            sdl2::keyboard::Keycode::AltErase => Keycode::AltErase,
            sdl2::keyboard::Keycode::Sysreq => Keycode::Sysreq,
            sdl2::keyboard::Keycode::Cancel => Keycode::Cancel,
            sdl2::keyboard::Keycode::Clear => Keycode::Clear,
            sdl2::keyboard::Keycode::Prior => Keycode::Prior,
            sdl2::keyboard::Keycode::Return2 => Keycode::Return2,
            sdl2::keyboard::Keycode::Separator => Keycode::Separator,
            sdl2::keyboard::Keycode::Out => Keycode::Out,
            sdl2::keyboard::Keycode::Oper => Keycode::Oper,
            sdl2::keyboard::Keycode::ClearAgain => Keycode::ClearAgain,
            sdl2::keyboard::Keycode::CrSel => Keycode::CrSel,
            sdl2::keyboard::Keycode::ExSel => Keycode::ExSel,
            sdl2::keyboard::Keycode::Kp00 => Keycode::Kp00,
            sdl2::keyboard::Keycode::Kp000 => Keycode::Kp000,
            sdl2::keyboard::Keycode::ThousandsSeparator => Keycode::ThousandsSeparator,
            sdl2::keyboard::Keycode::DecimalSeparator => Keycode::DecimalSeparator,
            sdl2::keyboard::Keycode::CurrencyUnit => Keycode::CurrencyUnit,
            sdl2::keyboard::Keycode::CurrencySubUnit => Keycode::CurrencySubUnit,
            sdl2::keyboard::Keycode::KpLeftParen => Keycode::KpLeftParen,
            sdl2::keyboard::Keycode::KpRightParen => Keycode::KpRightParen,
            sdl2::keyboard::Keycode::KpLeftBrace => Keycode::KpLeftBrace,
            sdl2::keyboard::Keycode::KpRightBrace => Keycode::KpRightBrace,
            sdl2::keyboard::Keycode::KpTab => Keycode::KpTab,
            sdl2::keyboard::Keycode::KpBackspace => Keycode::KpBackspace,
            sdl2::keyboard::Keycode::KpA => Keycode::KpA,
            sdl2::keyboard::Keycode::KpB => Keycode::KpB,
            sdl2::keyboard::Keycode::KpC => Keycode::KpC,
            sdl2::keyboard::Keycode::KpD => Keycode::KpD,
            sdl2::keyboard::Keycode::KpE => Keycode::KpE,
            sdl2::keyboard::Keycode::KpF => Keycode::KpF,
            sdl2::keyboard::Keycode::KpXor => Keycode::KpXor,
            sdl2::keyboard::Keycode::KpPower => Keycode::KpPower,
            sdl2::keyboard::Keycode::KpPercent => Keycode::KpPercent,
            sdl2::keyboard::Keycode::KpLess => Keycode::KpLess,
            sdl2::keyboard::Keycode::KpGreater => Keycode::KpGreater,
            sdl2::keyboard::Keycode::KpAmpersand => Keycode::KpAmpersand,
            sdl2::keyboard::Keycode::KpDblAmpersand => Keycode::KpDblAmpersand,
            sdl2::keyboard::Keycode::KpVerticalBar => Keycode::KpVerticalBar,
            sdl2::keyboard::Keycode::KpDblVerticalBar => Keycode::KpDblVerticalBar,
            sdl2::keyboard::Keycode::KpColon => Keycode::KpColon,
            sdl2::keyboard::Keycode::KpHash => Keycode::KpHash,
            sdl2::keyboard::Keycode::KpSpace => Keycode::KpSpace,
            sdl2::keyboard::Keycode::KpAt => Keycode::KpAt,
            sdl2::keyboard::Keycode::KpExclam => Keycode::KpExclam,
            sdl2::keyboard::Keycode::KpMemStore => Keycode::KpMemStore,
            sdl2::keyboard::Keycode::KpMemRecall => Keycode::KpMemRecall,
            sdl2::keyboard::Keycode::KpMemClear => Keycode::KpMemClear,
            sdl2::keyboard::Keycode::KpMemAdd => Keycode::KpMemAdd,
            sdl2::keyboard::Keycode::KpMemSubtract => Keycode::KpMemSubtract,
            sdl2::keyboard::Keycode::KpMemMultiply => Keycode::KpMemMultiply,
            sdl2::keyboard::Keycode::KpMemDivide => Keycode::KpMemDivide,
            sdl2::keyboard::Keycode::KpPlusMinus => Keycode::KpPlusMinus,
            sdl2::keyboard::Keycode::KpClear => Keycode::KpClear,
            sdl2::keyboard::Keycode::KpClearEntry => Keycode::KpClearEntry,
            sdl2::keyboard::Keycode::KpBinary => Keycode::KpBinary,
            sdl2::keyboard::Keycode::KpOctal => Keycode::KpOctal,
            sdl2::keyboard::Keycode::KpDecimal => Keycode::KpDecimal,
            sdl2::keyboard::Keycode::KpHexadecimal => Keycode::KpHexadecimal,
            sdl2::keyboard::Keycode::LCtrl => Keycode::LCtrl,
            sdl2::keyboard::Keycode::LShift => Keycode::LShift,
            sdl2::keyboard::Keycode::LAlt => Keycode::LAlt,
            sdl2::keyboard::Keycode::LGui => Keycode::LGui,
            sdl2::keyboard::Keycode::RCtrl => Keycode::RCtrl,
            sdl2::keyboard::Keycode::RShift => Keycode::RShift,
            sdl2::keyboard::Keycode::RAlt => Keycode::RAlt,
            sdl2::keyboard::Keycode::RGui => Keycode::RGui,
            sdl2::keyboard::Keycode::Mode => Keycode::Mode,
            sdl2::keyboard::Keycode::AudioNext => Keycode::AudioNext,
            sdl2::keyboard::Keycode::AudioPrev => Keycode::AudioPrev,
            sdl2::keyboard::Keycode::AudioStop => Keycode::AudioStop,
            sdl2::keyboard::Keycode::AudioPlay => Keycode::AudioPlay,
            sdl2::keyboard::Keycode::AudioMute => Keycode::AudioMute,
            sdl2::keyboard::Keycode::MediaSelect => Keycode::MediaSelect,
            sdl2::keyboard::Keycode::Www => Keycode::Www,
            sdl2::keyboard::Keycode::Mail => Keycode::Mail,
            sdl2::keyboard::Keycode::Calculator => Keycode::Calculator,
            sdl2::keyboard::Keycode::Computer => Keycode::Computer,
            sdl2::keyboard::Keycode::AcSearch => Keycode::AcSearch,
            sdl2::keyboard::Keycode::AcHome => Keycode::AcHome,
            sdl2::keyboard::Keycode::AcBack => Keycode::AcBack,
            sdl2::keyboard::Keycode::AcForward => Keycode::AcForward,
            sdl2::keyboard::Keycode::AcStop => Keycode::AcStop,
            sdl2::keyboard::Keycode::AcRefresh => Keycode::AcRefresh,
            sdl2::keyboard::Keycode::AcBookmarks => Keycode::AcBookmarks,
            sdl2::keyboard::Keycode::BrightnessDown => Keycode::BrightnessDown,
            sdl2::keyboard::Keycode::BrightnessUp => Keycode::BrightnessUp,
            sdl2::keyboard::Keycode::DisplaySwitch => Keycode::DisplaySwitch,
            sdl2::keyboard::Keycode::KbdIllumToggle => Keycode::KbdIllumToggle,
            sdl2::keyboard::Keycode::KbdIllumDown => Keycode::KbdIllumDown,
            sdl2::keyboard::Keycode::KbdIllumUp => Keycode::KbdIllumUp,
            sdl2::keyboard::Keycode::Eject => Keycode::Eject,
            sdl2::keyboard::Keycode::Sleep => Keycode::Sleep,
        }
    }
}