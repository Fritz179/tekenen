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

pub use tekenen::{Tekenen, colors, Pixel, Draw, TransforView, OverflowBehavior};

/// UI, Describe layout in a 'css' manner
pub mod ui;

/// Rust-embed
/// - emded files in executable
#[cfg(feature = "rust-embed")]
pub mod rust_embed {
    pub use rust_embed::*;

    pub trait DynRustEmbed {
        fn dyn_get(&self, file_path: &str) -> Option<EmbeddedFile>;
    }
}

pub mod shapes;

pub mod math;