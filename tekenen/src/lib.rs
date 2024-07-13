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