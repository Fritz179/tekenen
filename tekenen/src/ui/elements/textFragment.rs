

use crate::{ui::style::{LayoutContext}, Draw};

use super::DomElement;

#[derive(Debug)]
pub struct TextFragment {
    pub text: String,
    pub owner: Box<dyn DomElement>
}

impl TextFragment {
    pub fn new(text: &str, owner: Box<dyn DomElement>) -> Self{
        Self {
            text: text.to_owned(),
            owner
        }
    }

    fn split_text(&self, width: i32, context: &LayoutContext) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut current_width = 0;

        for token in self.text.split_whitespace() {
            let token_width = token.chars().count() as i32 * 16;

            if current_width + token_width > width {
                result.push(current);
                current = String::new();
                current_width = 0;
            }

            current.push_str(token);
            current.push(' ');
            current_width += token_width + 16;
        }

        if !current.is_empty() {
            result.push(current);
        }

        result
    }
}