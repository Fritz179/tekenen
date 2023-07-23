#![allow(dead_code)]

use tekenen::{Tekenen, colors, ui::*};
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/img/"]
struct Asset;

use image;
use image::GenericImageView;

fn load_image_asset<Asset: RustEmbed>(_: Asset, path: &str) -> Tekenen {
    let source = Asset::get(path).unwrap();
    let img = image::load_from_memory(&source.data).unwrap();

    let mut vec = vec![];

    for y in 0..img.height() {
        for x in 0..img.width() {
            let color = img.get_pixel(x, y);
            vec.push(color[0]);
            vec.push(color[1]);
            vec.push(color[2]);
            vec.push(color[3]);
        }
    };

    let width = img.width() as usize;
    let height = img.height() as usize;
 
    Tekenen::from_pixels(width, height, vec)
}

fn load_image_asset_bin<Asset: RustEmbed>(_: Asset, path: &str) -> Tekenen {
    let data = Asset::get(path).unwrap().data;
    assert!(data.len() >= 8);

    let (width, data) = data.split_at(4);
    let (height, data) = data.split_at(4);

    let width = u32::from_be_bytes(width.to_owned().try_into().unwrap()) as usize;
    let height = u32::from_be_bytes(height.to_owned().try_into().unwrap()) as usize;

    assert_eq!(data.len(), width * height * 4);

    Tekenen::from_pixels(width, height, data.to_owned())
}

fn main() {
    let mut window = Platform::new(800, 600).unwrap();
    let mut tekenen = Tekenen::new(800, 600);

    let mut tick = 0;

    let mut slider = widgets::Slider::new(300, 500, 50);

    let img8 = load_image_asset(Asset, "8.png"); 
    let img8 = load_image_asset_bin(Asset, "8.bin"); 

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

        // Platform::load_image("test");

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

        tekenen.draw_image(600, 200, &img8);
        tekenen.draw_scaled_image(600, 25, &img8, 5);

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

        // Draw slider
        slider.display(&mut tekenen);
        tekenen.draw_text(&format!("Value: {}", slider.value), 300, 75);

        window.display_pixels(tekenen.get_pixels());
        tick += 1;

        IntervalDecision::Repeat
    }, 60)
}
