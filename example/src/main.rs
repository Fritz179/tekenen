use tekenen::{Tekenen, colors, ui::*};
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event};

use tekenen::rust_embed;
use tekenen::rust_embed::{RustEmbed, DynRustEmbed};

#[derive(RustEmbed)]
#[folder = "src/img/"]
struct Asset;

impl DynRustEmbed for Asset {
    fn dyn_get(&self, file_path: &str) -> Option<rust_embed::EmbeddedFile> {
        Self::get(file_path)
    }
}

fn main() {
    let mut window = Platform::new(800, 600).unwrap();
    let mut tekenen = Tekenen::new(800, 600);

    Platform::set_assets(Asset);

    let mut tick = 0;

    let mut slider = widgets::Slider::new(300, 500, 50);

    let img8_png = Platform::load_image("8.png").unwrap(); 
    let img8_fpia = Platform::load_image("8.fpia").unwrap(); 

    Platform::set_interval(move || {
        while let Some(event) = window.read_events() {
            match event {
                Event::Quit => {
                    return IntervalDecision::Stop
                },
                Event::KeyDown { char: Some(char), .. } => {
                    println!("{char}")
                },
                Event::MouseDown { x, y } => {
                    slider.mouse_down(x, y);
                },
                Event::MouseMove { x, y } => {
                    slider.mouse_move(x, y);
                },
                Event::MouseUp { x, y } => {
                    slider.mouse_up(x, y);
                },
                _ => { }
            }
        }

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

        tekenen.draw_image(600, 200, &img8_png);
        tekenen.draw_scaled_image(600, 25, &img8_fpia, 5);

        tekenen.ui(Container::horiziontal(vec![
            Container::new(|_b, _tekenen| {
                // tekenen.rect(b.x, b.y, b.w, b.h, colors::RED)
            }),
            Container::vertical(vec![
                Container::new(|_b, _tekenen| {
                    // tekenen.rect(b.x, b.y, b.w, b.h, colors::WHITE)
                }),
                Container::new(|b, tekenen| {
                    tekenen.rect(b.x, b.y, b.w, b.h, colors::WHITE)
                }),
            ])
        ]));

        // Draw slider
        slider.display(&mut tekenen);
        tekenen.draw_text(&format!("Value: {}", slider.value), 300, 75);

        window.display_pixels(tekenen.get_pixels());

        tick += 1;

        IntervalDecision::Repeat
    }, 60)
}
