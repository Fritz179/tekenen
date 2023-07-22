#![allow(dead_code)]

// Platform contains all platform specific stuff.
//  - Timemanager
//  - Eventloop
//  - Files
pub mod platform;

// Tekenen contains all the drawing, platform independent stuff
// - background(), rect(), draw_image()
mod tekenen;
pub use tekenen::{Tekenen, colors, Pixel};

// UI, Describe layout in a 'css' manner
pub mod ui;