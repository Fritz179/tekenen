# Tekenen

Simple library for drawing pixels in memory.

IMPORTANT: This library is work in progress, everything is subject to change, use this library at your own risk.

## Basic Example

```rust
use tekenen::{Tekenen, colors};
use tekenen::platform::{Platform, PlatformTrait, Event, IntervalDecision};

fn main() {
    let mut window = Platform::new(800, 600).unwrap();
    let mut tek = Tekenen::new(800, 600);

    Platform::set_interval(move || {
        while let Some(event) = window.read_events() {
            match event {
                Event::Quit => {
                    return IntervalDecision::Stop
                },
                _ => { }
            }
        }

        tek.background(colors::GRAY);

        window.display_pixels(tek.get_pixels());

        IntervalDecision::Repeat
    }, 60)
}
```