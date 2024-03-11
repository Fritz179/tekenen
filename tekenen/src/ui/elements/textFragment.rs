

use std::cell::Ref;

use crate::{colors, math::{IndefRange, Vec2}, shapes::rect::Rect, ui::style::{FormattingInfo, Style}, Draw, Font, Tekenen, Wrapper};

use super::{DomElement, FormattingContext, InlineFormattingContext, LayoutBox, LayoutNode, LineBox, PaintElement, PainterTree, Stylable};

#[derive(Debug)]
pub struct InnerTextFragment {
    pub text: String,
    pub owner: Box<dyn DomElement>
}

pub type TextFragment = Wrapper<InnerTextFragment>;

impl TextFragment {
    pub fn new(text: &str, owner: Box<dyn DomElement>) -> Box<Self> {
        Wrapper::wrap(InnerTextFragment {
            text: text.to_owned(),
            owner
        })
    }

    fn split_text(&self, width: i32, context: &FormattingInfo) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut current_width = 0;

        for token in self.borrow().text.split_whitespace() {
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

impl Stylable for TextFragment {
    fn get_style(&self) -> Ref<'_, Style> {
        todo!()
    }

    fn get_name(&self) -> String {
        format!("TextFragment: {}", self.borrow().text)
    }
}

impl TextFragment {
    pub fn get_layout_box(&self) -> LayoutNode {
        LayoutNode::new(self.clone())
    }
}

impl LayoutBox for TextFragment {
    fn get_height_from_width(&self, width: i32, context: &FormattingInfo) -> i32 {
        16
    }

    fn get_width_from_height(&self, height: i32, context: &FormattingInfo) -> i32 {
        todo!()
    }

    fn get_inner_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_min_max_content(&self, context: FormattingInfo) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_painter(&self, content_box: Rect, context: &FormattingInfo) -> Box<dyn PaintElement> {
        self.clone()
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn go_inline_yourself(&self, formatter: &mut InlineFormattingContext, context: &dyn super::FormattingContext, info: &FormattingInfo) -> Vec<(Box<LineBox>, Box<dyn LayoutBox>)> {
        let (line, _) = formatter.get_new_line(context, info);

        vec![(line.add(self.clone()), self.clone())]
    }
}

impl PaintElement for TextFragment {
    fn draw(&self, target: &mut Tekenen, context: &FormattingInfo, space: Vec2) {
        for (i, line) in self.split_text(space.x, context).iter().enumerate() {
            target.text(&line, 0, i as i32 * 16, Font::new(16, colors::MAGENTA));
        }
    }
}