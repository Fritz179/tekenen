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

use std::{borrow::Borrow, cell::{Ref, RefCell, RefMut}, ops::Deref, rc::Rc};

pub use tekenen::{Tekenen, colors, Pixel, Draw, OverflowBehavior, Font};

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

#[derive(Debug)]
pub struct Wrapper<T>(Rc<RefCell<T>>);

impl<T> Wrapper<T> {
    fn wrap(t: T) -> Box<Wrapper<T>> {
        Box::new(Wrapper::<T> {
            0: Rc::new(RefCell::new(t))
        })
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.as_ref().borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.as_ref().borrow_mut()
    }

    pub fn clone(&self) -> Box<Wrapper<T>> {
        Box::new(Wrapper::<T> {0: Rc::clone(&self.0)})
    }
}
