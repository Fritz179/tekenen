use crate::{colors, math::{IndefRange, Range, Vec2}, ui::style::{Context, Style}, Draw, Pixel, Tekenen};
use super::Element;

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
    style: Style
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
            style: Style::default()
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
            style: Style::default()
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

    fn get_width_from_height(&self, height: i32, context: &Context) -> i32 {
        self.width
    }

    fn get_height_from_width(&self, width: i32, context: &Context) -> i32 {
        self.knob_radius * 2
    }

    fn get_inner_min_max_content(&self, context: &Context) -> Vec2<IndefRange> {
        Vec2::new(IndefRange::new_definite(self.width), IndefRange::new_definite(self.knob_radius * 2))
    }

    fn draw(&self, target: &mut Tekenen, context: &Context, space: Vec2) {
        target.rect(0, 0, self.width, self.knob_radius * 2, self.slider_color);
        target.circle(self.current, self.knob_radius, self.knob_radius, self.knob_color);

        Vec2::new(self.width, self.knob_radius * 2);
    }

    fn get_style(&self) -> &Style {
        &self.style
    }
}