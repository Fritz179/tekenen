

use std::cell::Ref;

use crate::{colors, math::{IndefRange, Vec2}, shapes::rect::Rect, ui::style::{FormattingInfo, Style}, Draw, Font, Tekenen, WeakWrapper, Wrapper};

use super::{DomElement, FormattingContext, InlineFormattingContext, LayoutBox, LayoutNode, LineBox, PaintElement, PainterTree, Stylable};

#[derive(Debug)]
pub struct InnerTextFragment {
    pub text: String,
    // pub owner: Box<WeakWrapper<dyn DomElement>>
}

pub type TextFragement = Wrapper<InnerTextFragment>;

impl TextFragement {
    pub fn new(text: &str) -> Box<Self> {
        Wrapper::wrap(InnerTextFragment {
            text: text.to_owned(),
            // owner
        })
    }
}

impl Stylable for TextFragement {
    fn get_style(&self) -> Ref<'_, Style> {
        todo!()
    }

    fn get_name(&self) -> String {
        format!("TextFragment: {}", self.borrow().text)
    }
}

impl LayoutBox for TextFragement {
    fn get_height_from_width(&self, width: i32, context: &FormattingInfo) -> i32 {
        16
    }

    fn get_width_from_height(&self, height: i32, context: &FormattingInfo) -> i32 {
        self.borrow().text.len() as i32 * 16
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
        todo!()
    }

    fn go_inline_yourself(&self, formatter: &mut InlineFormattingContext, context: &dyn super::FormattingContext, info: &FormattingInfo) -> Vec<(Box<LineBox>, Box<dyn LayoutBox>)> {
        todo!()
    }

}

impl PaintElement for TextFragement {
    fn draw(&self, target: &mut Tekenen, context: &FormattingInfo, space: Vec2) {
        target.text(&self.borrow().text, 0, 0, Font::new(16, colors::WHITE));
    }
}

#[derive(Debug)]
pub struct InnerTextNode {
    pub text: String,
    // pub owner: Box<WeakWrapper<dyn DomElement>>
}

pub type TextNode = Wrapper<InnerTextNode>;

impl TextNode {
    pub fn new(text: &str) -> Box<Self> {
        Wrapper::wrap(InnerTextNode {
            text: text.to_owned(),
            // owner
        })
    }

    // fn split_text(&self, width: i32, context: &FormattingInfo) -> Vec<String> {
    //     let mut result = Vec::new();
    //     let mut current = String::new();
    //     let mut current_width = 0;

    //     for token in self.borrow().text.split_whitespace() {
    //         let token_width = token.chars().count() as i32 * 16;

    //         if current_width + token_width > width {
    //             result.push(current);
    //             current = String::new();
    //             current_width = 0;
    //         }

    //         current.push_str(token);
    //         current.push(' ');
    //         current_width += token_width + 16;
    //     }

    //     if !current.is_empty() {
    //         result.push(current);
    //     }

    //     result
    // }
}

impl Stylable for TextNode {
    fn get_style(&self) -> Ref<'_, Style> {
        todo!()
    }

    fn get_name(&self) -> String {
        format!("TextNode: {}", self.borrow().text)
    }
}

impl DomElement for TextNode {
    fn event(&mut self, event: crate::platform::Event) {
        todo!()
    }

    fn get_dom_children(&self) -> Option<Ref<'_, Vec<Box<dyn DomElement>>>> {
        todo!()
    }

    fn get_layout_box(&self) -> LayoutNode {
        LayoutNode::new(self.clone())
    }

    fn update(&mut self) {
        todo!()
    }
}

impl LayoutBox for TextNode {
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
        let mut current_line = formatter.get_line(context, info).0;
        let mut lines: Vec<(Box<_>, Box<dyn LayoutBox>)> = Vec::new();

        let mut current_string = String::new();
        let mut current_width = 0;

        for token in self.borrow().text.split_whitespace() {
            let token_width = token.chars().count() as i32 * 16;

            // Dispatch line if the token does not fit
            if current_width + token_width > current_line.available_width() && !current_string.is_empty() {
                let fragment = TextFragement::new(&current_string);
                lines.push((current_line.add(fragment.clone()), fragment.clone()));

                // Reset
                current_line = formatter.get_new_line(context, info).0;
                current_string = String::new();
                current_width = 0;
            }

            current_string.push_str(token);
            current_string.push(' ');
            current_width += token_width + 16;
        }

        // Dispatch the last line
        if !current_string.is_empty() {
            let fragment = TextFragement::new(&current_string);
            lines.push((current_line.add(fragment.clone()), fragment.clone()));
        }

        lines
    }
}

impl PaintElement for TextNode {
    fn draw(&self, target: &mut Tekenen, context: &FormattingInfo, space: Vec2) {
        // for (i, line) in self.split_text(space.x, context).iter().enumerate() {
        //     target.text(&line, 0, i as i32 * 16, Font::new(16, colors::MAGENTA));
        // }
    }
}