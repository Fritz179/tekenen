use demo::Demo;
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event};

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

// Demo ideas:
//  1) Font rendering
//  2) CSS style rendering
//  3) Physics demo
//  4) SubCanvas


fn main() {
    Platform::set_assets(Asset);

    let mut window = Box::new(Platform::new(800, 600).unwrap());

    // let mut demo = demo::basic::BasicDemo::new();
    // let mut demo = demo::text::TextDemo::new();
    let mut demo = demo::transform::TransfromDemo::new();

    Platform::set_interval(move || {
        while let Some(event) = window.read_events() {
            if let Event::Quit = event {
                return IntervalDecision::Stop
            }

            match demo.update(event) {
                IntervalDecision::Repeat => {},
                IntervalDecision::Stop => return IntervalDecision::Stop
            }
        }

        demo.draw(&mut window);

        IntervalDecision::Repeat
    }, 60)
}