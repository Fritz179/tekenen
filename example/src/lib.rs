use tekenen::{colors, platform::{Event, IntervalDecision, KeyDownEvent, Platform, PlatformTrait}, Draw, Tekenen};

use wasm_bindgen::prelude::*;

mod demo;
use demo::Demo;

#[wasm_bindgen]
pub fn wasm_start() {
    main()
}

pub fn main() {
    let mut window = Box::new(Platform::new(800, 600).unwrap());

    let mut demos: Vec<Box<dyn Demo>> = vec![
        Box::new(demo::image::ImageDemo::new()),
        Box::new(demo::canvas::CanvasDemo::new()),
    ];

    let mut current_demo = 0;

    Platform::set_interval(move || {
        while let Some(event) = window.read_events() {

            // Check if we have to react to an event
            match event {
                Event::Quit => return IntervalDecision::Stop,
                Event::KeyDown(KeyDownEvent { char: Some('n'), .. }) => {
                    current_demo += 1;

                    if current_demo >= demos.len() {
                        current_demo = 0;
                        continue
                    }
                },
                _ => { }
            }

            // Send the event forward to the current demo
            match demos[current_demo].update(&event) {
                IntervalDecision::Repeat => { },
                IntervalDecision::Stop => return IntervalDecision::Stop
            }
        }

        demos[current_demo].draw(&mut window);

        IntervalDecision::Repeat
    }, 60)
}