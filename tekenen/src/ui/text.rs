use super::UIBox;

pub struct Text {
    text: String
}

impl Text {
    pub fn new(text: &str) -> Box<Self> {
        Box::new(Self {
            text: text.to_owned()
        })
    }
}

impl UIBox for Text {
    fn draw(&mut self, view: super::ViewBox, tek: &mut crate::Tekenen) {
        tek.draw_text(&self.text, view.x, view.y);
    }
}