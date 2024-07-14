#![allow(dead_code)]
#![allow(unused_variables)]

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



pub use tekenen::{Tekenen, colors, Pixel, Draw, OverflowBehavior, Font};

/// UI, Describe layout in a 'css' manner
pub mod html;

pub mod shapes;

pub mod math;

#[cfg(all(feature = "server", not(target_family = "wasm")))]
pub fn server() {
    use rouille;

    println!("\nNow listening on `https://localhost:8000`");

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
    
            std::process::Command::new("wasm-pack")
                .args([
                    "build",
                    "./",
                    "--target",
                    "web",
                    "--out-dir",
                    "./home/wasm",
                ])
                .status()
                .expect("failed to build wasm");
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

