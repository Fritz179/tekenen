use tekenen::{Platform, PlatformTrait, IntervalDecision, Event, Tekenen, colors};

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", message());

    let mut window = Platform::new(800, 600).unwrap();
    let mut tekenen = Tekenen::new(800, 600);

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


        tekenen.background(colors::GRAY);

        tekenen.rect(50, 100, 100, 150, colors::BLACK);
        tekenen.circle(150, 100, 50, colors::RED);

        tekenen.line(50, 100, 150, 250, colors::WHITE);

        tekenen.line(300, 300, 350, 350, colors::WHITE);
        tekenen.line(350, 300, 400, 300, colors::WHITE);
        tekenen.line(400, 300, 450, 250, colors::WHITE);

        tekenen.draw_text("Hello ff", 200, 200);

        // println!("{:?}", Platform::get_remaining_time());

        window.display_pixels(tekenen.get_pixels());

        IntervalDecision::Repeat
    }, 60)
}
