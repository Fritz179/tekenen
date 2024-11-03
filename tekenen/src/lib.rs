#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(slice_flatten)]
#![feature(array_chunks)]
#![feature(trait_upcasting)]

#[cfg(feature = "preloader")]
pub mod preloader;

/// Platform contains all platform specific stuff.
///  - Timemanager
///  - Eventloop
/// - Files
pub mod platform;

/// Tekenen contains all the drawing, platform independent stuff.
/// - background(), rect(), draw_image()
mod tekenen;



pub use tekenen::{SurfaceView, Surface, colors, Pixel, DrawableSurface, OverflowBehavior};

/// UI, Describe layout in a 'css' manner
// pub mod html;
pub mod fui;

pub mod shapes;

pub mod math;

pub mod printer;

#[cfg(all(feature = "server", not(target_family = "wasm")))]
pub fn server() {
    use rouille;

    println!("\nNow listening on `https://localhost:8000/index.html`");

    rouille::start_server("localhost:8000", move |request| {
        let response = rouille::match_assets(request, "./example/home");
    
        if response.is_success() {
            return response;
        }
    
        rouille::Response::html(
            "404 error: The requested page could not be found",
        )
        .with_status_code(404)
    });
}

#[cfg(feature = "wasm")]
#[macro_export]
macro_rules! BUILD_WASM {
    () => {
        let target = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap();

        if target == "wasm" {
            println!("cargo:warning=Skipping wasm because target is not unix");
        } else {
            println!("cargo:warning=Building wasm");
            let path = "./home/wasm";

            // remove the old wasm files
            let _ = std::fs::remove_dir_all(path);
            fs::create_dir(path).unwrap();
    
            use std::process::Command;
            let status = Command::new("wasm-pack")
                .args([
                    "build",
                    "./",
                    "--target",
                    "web",
                    "--out-dir",
                    path,
                ])
                .output()
                .unwrap();

            let stdout = std::str::from_utf8(&status.stdout).unwrap().to_owned();
            let stderr = std::str::from_utf8(&status.stderr).unwrap().to_owned();

            let data = format!("wasm-pack STDOUT:\n{}\nwasm-pack STDERR:\n{}", stdout, stderr);

            // Display error message in terminal
            for line in data.lines() {
                println!("cargo:warning={}", line);
            }

            // Test output dir is empty
            let directory = std::fs::read_dir(path).unwrap();

            if directory.count() == 0 || data.lines().count() > 15 {
                // Display error message in the browser
                fs::write(path.to_owned() + "/example.js", format!("
                    export default function init() {{ 
                        window.js_log(`{}`);
                        return {{ then: _ => {{}} }};
                    }}
                ", data.to_owned().replace("`", "\\`"))).unwrap();
            }
        }
    };
}

#[cfg(not(feature = "wasm"))]
#[macro_export]
macro_rules! BUILD_WASM {
    () => {
        println!("cargo:warning=Skipping wasm because it's not enabled");
    };
}

