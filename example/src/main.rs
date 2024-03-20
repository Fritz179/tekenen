


use demo::Demo;
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event, KeyDownEvent};

use tekenen::rust_embed;
use tekenen::rust_embed::{RustEmbed, DynRustEmbed, EmbeddedFile};

#[derive(RustEmbed)]
#[folder = "src/img/"]
struct Asset;

impl DynRustEmbed for Asset {
    fn dyn_get(&self, file_path: &str) -> Option<EmbeddedFile> {
        Self::get(file_path)
    }
}

mod demo;
mod test;

static mut DEBUG_MODE: bool = false;


fn main() {
    test::run();

    Platform::set_assets(Asset);

    let mut window = Box::new(Platform::new(800, 600).unwrap());

    let mut demos: Vec<Box<dyn Demo>> = vec![
        Box::new(demo::image::ImageDemo::new()),
        Box::new(demo::canvas::CanvasDemo::new()),
        Box::new(demo::float::FloatDemo::new()),
        // Box::new(demo::text::TextDemo::new()),
    ];

    let mut current_demo = 0;

    Platform::set_interval(move || {
        while let Some(event) = window.read_events() {

            match event {
                Event::Quit => return IntervalDecision::Stop,
                Event::KeyDown(KeyDownEvent { char: Some('n'), .. }) => {
                    current_demo += 1;

                    if current_demo >= demos.len() {
                        current_demo = 0;
                        continue
                    }
                },
                Event::KeyDown(KeyDownEvent { char: Some('m'), .. }) => {
                    unsafe {
                        DEBUG_MODE = !DEBUG_MODE
                    }
                }
                _ => { }
            }

            match demos[current_demo].update(&event) {
                IntervalDecision::Repeat => {},
                IntervalDecision::Stop => return IntervalDecision::Stop
            }
        }

        demos[current_demo].draw(&mut window);

        IntervalDecision::Repeat
    }, 60)
}