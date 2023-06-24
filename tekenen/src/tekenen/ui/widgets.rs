use crate::{Pixel, colors, Tekenen};

pub struct Scroller {
    pub x1: i32,
    pub x2: i32,
    pub y: i32,
    pub x: i32,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub slider_width: i32,
    pub slider_color: Pixel,
    pub knob_radius: i32,
    pub know_color: Pixel,
    pub scrolling: bool,
}

impl Scroller {
    pub fn new(x1: i32, x2: i32, y: i32) -> Self {
        Self {
            x1,
            x2,
            x: (x1 + x2) / 2,
            y,
            min: 0.0,
            max: 1.0,
            value: 0.5,
            slider_width: 10,
            slider_color: colors::WHITE,
            knob_radius: 20,
            know_color: colors::RED,
            scrolling: false,
        }
    }

    fn update_position(&mut self, x: i32) {
        self.x = x;
        if self.x > self.x2 { self.x = self.x2 }
        if self.x < self.x1 { self.x = self.x1 }

        self.value = self.min + (self.max - self.min) * (self.x - self.x1) as f32 / (self.x2 - self.x1) as f32;
    }

    pub fn mouse_down(&mut self, x: i32, y: i32) {
        let dx = x - self.x;
        let dy = y - self.y;

        if dx*dx + dy*dy < self.knob_radius*self.knob_radius {
            self.scrolling = true;
        }
    }

    pub fn mouse_up(&mut self, x: i32, y: i32) {
        if self.scrolling {
            self.scrolling = false;
            self.update_position(x);
        }
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        if self.scrolling {
            self.update_position(x);
        }
    }

    pub fn display(&self, tekenen: &mut Tekenen) {
        tekenen.rect(self.x1, self.y - self.slider_width / 2, self.x2 - self.x1, self.slider_width, self.slider_color);
        tekenen.circle(self.x, self.y, self.knob_radius, self.know_color);
    }
}