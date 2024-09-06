use std::{cell::{Ref, RefCell}, collections::VecDeque};


use crate::Surface;

use super::{PlatformTrait, PlatformError, Event, KeyDownEvent, MouseKey, Keymod, IntervalDecision};

use wasm_bindgen::prelude::*;

pub struct WASMPlatform {}

type Callback = Box<dyn FnMut() -> IntervalDecision>;

thread_local! {
    static ACTIVE_CALLBACK: RefCell<Option<Callback>> = RefCell::new(None);
    static KEY_QUEUE: RefCell<VecDeque<Event>> = RefCell::new(VecDeque::new());
}

impl PlatformTrait for WASMPlatform {
    fn new(width: u32, height: u32) -> Result<Self, PlatformError>
        where
    Self: Sized {
        js_set_size(width, height);

        Ok(WASMPlatform {})
    }

    fn display_surface(&mut self, surface: Ref<Surface>) {
        // TODO: Use shared array buffers!!
        js_display_pixels(surface.pixels.clone().flatten().to_vec().into_boxed_slice())
    }

    fn read_events(&mut self) -> Option<Event> {
        KEY_QUEUE.with(|queue| {
            let mut queue = queue.borrow_mut();
            queue.pop_front()
        })
    }

    fn set_interval(callback: impl FnMut() -> IntervalDecision + 'static, fps: u32) {
        ACTIVE_CALLBACK.with(|active| {
            let mut active = active.borrow_mut();

            if active.is_some() {
                panic!("Only one interval supported");
            } else {
                // todo!("Set interval")
                *active = Some(Box::new(callback));
            }
        });

        js_set_interval(fps)
    }

    fn get_remaining_time() -> std::time::Duration {
        todo!()
    }

    fn log(value: String) {
        js_log(value)
    }
}

#[wasm_bindgen]
pub fn wasm_key_down(key: char) {
    KEY_QUEUE.with(|queue| {
        let mut queue = queue.borrow_mut();

        let event = Event::KeyDown(KeyDownEvent {
            repeat: false,
            char: Some(key),
            keycode: None,
            keymod: Keymod {
                shift: false,
                ctrl: false,
                caps: false,
            },
        });

        queue.push_back(event)
    })
}

pub fn mouse_key_to_keycode(key: i32) -> MouseKey {
    match key {
        0 => MouseKey::Left,
        1 => MouseKey::Middle,
        2 => MouseKey::Right,
        3 => MouseKey::Back,
        4 => MouseKey::Forward,
        _ => unreachable!(),
    }
}

#[wasm_bindgen]
pub fn wasm_mouse_down(x: i32, y: i32, button: i32) {
    KEY_QUEUE.with(|queue| {
        let mut queue = queue.borrow_mut();

        let event = Event::MouseDown {
            x,
            y,
            key: mouse_key_to_keycode(button),
        };

        queue.push_back(event)
    })
}

#[wasm_bindgen]
pub fn wasm_mouse_move(x: i32, y: i32, xd: i32, yd: i32) {
    KEY_QUEUE.with(|queue| {
        let mut queue = queue.borrow_mut();

        let event = Event::MouseMove {
            x,
            y,
            xd,
            yd,
        };

        queue.push_back(event)
    })
}

#[wasm_bindgen]
pub fn wasm_mouse_up(x: i32, y: i32, button: i32) {
    KEY_QUEUE.with(|queue| {
        let mut queue = queue.borrow_mut();

        let event = Event::MouseUp {
            x,
            y,
            key: mouse_key_to_keycode(button),
        };

        queue.push_back(event)
    })
}

#[wasm_bindgen]
pub fn wasm_run_callback() {
    ACTIVE_CALLBACK.with(|active| {
        let mut active = active.borrow_mut();

        let active = active.as_mut();

        if let Some(active) = active {
            let _should_stop = active();
        } else {
            panic!("No callback set!");
        }
    })
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn js_log(value: String);

    #[wasm_bindgen]
    fn js_set_size(width: u32, height: u32);

    #[wasm_bindgen]
    fn js_set_interval(fps: u32);

    #[wasm_bindgen]
    fn js_display_pixels(pixels: Box<[u8]>);
}
