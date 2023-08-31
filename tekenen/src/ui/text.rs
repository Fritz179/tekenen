use super::{UIBox, BoundingBox};
use crate::Draw;

pub struct Text {
    text: String,
    bounding_box: BoundingBox
}

impl Text {
    pub fn new(text: &str) -> Box<Self> {
        Box::new(Self {
            text: text.to_owned(),
            bounding_box: BoundingBox::default()
        })
    }
}

impl UIBox for Text {
    fn draw(&mut self, tv: &mut dyn Draw) {
        tv.draw_text(&self.text, 0, 0);
    }

    fn get_box(&mut self, max: BoundingBox) -> &BoundingBox {
        self.bounding_box = BoundingBox::new(16 * self.text.len() as i32, 20);
        &self.bounding_box
    }
}