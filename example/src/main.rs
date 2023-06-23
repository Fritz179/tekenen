use tekenen::{Platform, PlatformTrait, IntervalDecision, Event, Tekenen, COLORS};

fn main() {
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


        tekenen.background(COLORS::GRAY);

        tekenen.rect(50, 100, 100, 150, COLORS::BLACK);
        tekenen.circle(150, 100, 50, COLORS::RED);

        tekenen.line(50, 100, 150, 250, COLORS::WHITE);

        tekenen.line(300, 300, 350, 350, COLORS::WHITE);
        tekenen.line(350, 300, 400, 300, COLORS::WHITE);
        tekenen.line(400, 300, 450, 250, COLORS::WHITE);

        tekenen.draw_text("Hello", 200, 200);

        println!("{:?}", Platform::get_remaining_time());

        window.display_pixels(tekenen.get_pixels());

        IntervalDecision::Repeat
    }, 60)
}
