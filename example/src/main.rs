#![allow(dead_code)]

use tekenen::{Platform, PlatformTrait, IntervalDecision, Event, Tekenen, colors, ui::*};

mod preloaded;

fn main() {
    let preloaded = preloaded::load_preloaded();

    let mut window = Platform::new(800, 600).unwrap();
    let mut tekenen = Tekenen::new(800, 600);

    let mut tick = 0;

    Platform::set_interval(move || {
        while let Some(event) = window.read_events() {
            match event {
                Event::Quit => {
                    return IntervalDecision::Stop
                },
                Event::KeyDown { char: Some(char), .. } => {
                    println!("{char}")
                },
                _ => { }
            }
        }

        // Tekenen is only a view on pixels?
            // - Easy to take subcanvas
            // - Harder to keep track of owner => Weak/Strong

        // Vec of Pixels or Vec of Rows of Pixels?
            // If its a 1D Vec => Stride
            // If its a 2D Vec => More Vectors
            // Resizing => Stride changes?

        // Every view is a different canvas?

        // You just get a bounding box with x,y,w,h?

        tekenen.background(colors::GRAY);

        tekenen.rect(50, 100, 100, 150, colors::BLACK);
        tekenen.circle(150, 100, 50, colors::RED);

        tekenen.line(50, 100, 150, 250, colors::WHITE);

        tekenen.line(300, 300, 350, 350, colors::WHITE);
        tekenen.line(350, 300, 400, 300, colors::WHITE);
        tekenen.line(400, 300, 450, 250, colors::WHITE);

        tekenen.line(100, 400, 100, 350, colors::WHITE);
        tekenen.line(150, 400, 150, 550, colors::WHITE);

        tekenen.draw_text(&format!("Hello there, tick: {tick}"), 200, 200);

        tekenen.draw_image(400, 400, &preloaded.img8);

        tekenen.ui(Container::horiziontal(vec![
            Container::new(|b, tekenen| {
                // tekenen.rect(b.x, b.y, b.w, b.h, colors::RED)
            }),
            Container::vertical(vec![
                Container::new(|b, tekenen| {
                    // tekenen.rect(b.x, b.y, b.w, b.h, colors::WHITE)
                }),
                Container::new(|b, tekenen| {
                    tekenen.rect(b.x, b.y, b.w, b.h, colors::WHITE)
                }),
            ])
        ]));

        window.display_pixels(tekenen.get_pixels());
        tick += 1;

        IntervalDecision::Repeat
    }, 60)
}
