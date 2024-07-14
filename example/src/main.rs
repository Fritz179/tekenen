use std::process::Command;

mod demo;
use rouille::Response;

fn main() {
    demo::main();

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