use crate::{colors, math::Vec2, Pixel, Tekenen, Draw};
use super::{Element, SpaceContraint};

#[derive(Debug)]
pub struct Slider {
    width: i32,
    current: i32,
    min: f32,
    max: f32,
    pub value: f32,
    pub slider_width: i32,
    pub slider_color: Pixel,
    pub knob_radius: i32,
    pub knob_color: Pixel,
    scrolling: bool,
}

impl Slider {
    pub fn new(width: i32) -> Box<Self> {
        Box::new(Self {
            width,
            current: width / 2,
            min: 0.0,
            max: 1.0,
            value: 0.5,
            slider_width: 10,
            slider_color: colors::WHITE,
            knob_radius: 20,
            knob_color: colors::RED,
            scrolling: false,
        })
    }

    pub fn new_sized(width: i32, min: f32, max: f32, value: f32) -> Box<Self> {
        Box::new(Self {
            width,
            current: (width as f32 * (value - min) / (max - min)) as i32,
            min,
            max,
            value,
            slider_width: 10,
            slider_color: colors::WHITE,
            knob_radius: 20,
            knob_color: colors::RED,
            scrolling: false,
        })
    }

    fn update_position(&mut self, position: i32) {
        self.current = position;

        if self.current > self.width { self.current = self.width }
        if self.current < 0 { self.current = 0 }

        self.value = self.min + (self.max - self.min) * self.current as f32 / self.width as f32;
    }

    fn mouse_down(&mut self, x: i32, y: i32) {
        let dx = x - self.current;
        let dy = y - self.knob_radius;

        if dx*dx + dy*dy < self.knob_radius*self.knob_radius {
            self.scrolling = true;
        }
    }

    fn mouse_up(&mut self, x: i32, _y: i32) {
        if self.scrolling {
            self.scrolling = false;
            self.update_position(x);
        }
    }

    fn mouse_move(&mut self, x: i32, _y: i32) {
        if self.scrolling {
            self.update_position(x);
        }
    }
}

impl Element for Slider {
    fn event(&mut self, event: crate::platform::Event) {
        match event {
            crate::platform::Event::MouseDown { x, y, .. } => self.mouse_down(x, y),
            crate::platform::Event::MouseUp { x, y, .. } => self.mouse_up(x, y),
            crate::platform::Event::MouseMove { x, y, .. } => self.mouse_move(x, y),
            _ => {}
        }
    }

    fn update(&mut self) {
        
    }

    fn get_layout(&self) -> SpaceContraint {
        SpaceContraint::new_fixed(self.width, self.knob_radius * 2, (self.width, self.width))
    }

    fn draw(&self, target: &mut Tekenen, available_space: Vec2) -> Vec2 {
        target.rect(0, 0, self.width, self.knob_radius * 2, self.slider_color);
        target.circle(self.current, self.knob_radius, self.knob_radius, self.knob_color);

        Vec2::new(self.width, self.knob_radius * 2)
    }
}