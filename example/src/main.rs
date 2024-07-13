use std::process::Command;

use demo::Demo;
use tekenen::platform::{Platform, PlatformTrait, IntervalDecision, Event, KeyDownEvent};

mod demo;
use rouille::Response;

fn main() {
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
    }, 60);

    Command::new("wasm-pack")
        // .args(["build", "../../wasm", "--target", "web"])
        .args([
            "build",
            "./example",
            "--target",
            "web",
            "--out-dir",
            "./home/wasm",
        ])
        // .args(["build", "../wasm", "--target", "web", "--out-dir", ])
        .status()
        .expect("failed to build wasm");

    #[cfg(not(target_family = "wasm"))]
    println!("Visit `http://localhost:8000/index.html`");

    #[cfg(not(target_family = "wasm"))]
    rouille::start_server("localhost:8000", move |request| {
        let response = rouille::match_assets(request, "./example/home");

        if response.is_success() {
            return response;
        }

        Response::html(
            "404 error: The requested page could not be found",
        )
        .with_status_code(404)
    });
}