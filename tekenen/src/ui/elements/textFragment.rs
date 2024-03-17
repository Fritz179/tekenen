

use std::{cell::{Ref, RefCell}, rc::Rc};

use crate::{colors, math::{IndefRange, Vec2}, shapes::rect::Rect, ui::style::{FormattingInfo, Style}, Draw, Font, Tekenen};

use super::{DomElement, FormattingContext, InlineFormattingContext, LayoutBox, LayoutNode, LineBox, PaintElement, PainterTree, Stylable};

#[derive(Debug)]
pub struct TextFragment {
    pub text: RefCell<String>,
}


impl TextFragment {
    pub fn new(text: &str) -> Rc<Self> {
        Rc::new(Self {
            text: RefCell::new(text.to_owned()),
            // owner
        })
    }
}

impl Stylable for TextFragment {
    fn get_style(&self) -> &std::cell::RefCell<Style> {
        todo!()
    }

    fn get_name(&self) -> String {
        format!("TextFragment: {}", self.text.borrow())
    }
}

impl LayoutBox for TextFragment {
    fn get_height_from_width(&self, width: i32, context: &FormattingInfo) -> i32 {
        16
    }

    fn get_width_from_height(&self, height: i32, context: &FormattingInfo) -> i32 {
        self.text.borrow().len() as i32 * 16
    }

    fn get_inner_min_max_content(&self, context: &FormattingInfo) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_min_max_content(&self, context: FormattingInfo) -> Vec2<IndefRange> {
        todo!()
    }

    fn get_painter(self: Rc<Self>, content_box: Rect, context: &FormattingInfo) -> Rc<dyn PaintElement> {
        self
    }

    fn is_inline(&self) -> bool {
        todo!()
    }

    fn go_inline_yourself(&self, formatter: &mut InlineFormattingContext, context: &dyn FormattingContext, info: &FormattingInfo) 
            -> Vec<(Rc<LineBox>, Rc<dyn LayoutBox>)> {
        todo!()
    }

}

impl PaintElement for TextFragment {
    fn draw(&self, target: &mut Tekenen, context: &FormattingInfo, space: Vec2) {
        target.text(&self.text.borrow(), 0, 0, Font::new(16, colors::WHITE));
    }
}

#[derive(Debug)]
pub struct TextNode {
    pub text: String,
    // pub owner: Box<WeakWrapper<dyn DomElement>>
}

impl TextNode {
    pub fn new(text: &str) -> Rc<Self> {
        Rc::new(Self {
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
    fn get_style(&self) -> &RefCell<Style> {
        todo!()
    }

    fn get_name(&self) -> String {
        format!("TextNode: {}", self.text)
    }
}

impl DomElement for TextNode {
    fn event(&mut self, event: crate::platform::Event) {
        todo!()
    }

    fn get_dom_children(&self) -> Option<&RefCell<Vec<Rc<dyn DomElement>>>> {
        None
    }

    fn get_layout_box(self: Rc<Self>) -> Rc<dyn LayoutBox> {
        self
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

    fn get_painter(self: Rc<Self>, content_box: Rect, context: &FormattingInfo) -> Rc<dyn PaintElement> {
        self
    }

    fn is_inline(&self) -> bool {
        true
    }

    fn go_inline_yourself(&self, formatter: &mut InlineFormattingContext, context: &dyn super::FormattingContext, info: &FormattingInfo) -> Vec<(Rc<LineBox>, Rc<dyn LayoutBox>)> {
        let mut current_line = formatter.get_line(context, info).0;
        let mut lines: Vec<(Rc<LineBox>, Rc<dyn LayoutBox>)> = Vec::new();

        let mut current_string = String::new();
        let mut current_width = 0;

        for token in self.text.split_whitespace() {
            let token_width = token.chars().count() as i32 * 16;

            // Dispatch line if the token does not fit
            if current_width + token_width > current_line.available_width() && !current_string.is_empty() {
                let fragment = TextFragment::new(&current_string) as Rc<dyn LayoutBox>;
                current_line.add(Rc::clone(&fragment));
                lines.push((Rc::clone(&current_line), Rc::clone(&fragment)));

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
            let fragment = TextFragment::new(&current_string);
            current_line.add(fragment.clone());
            lines.push((Rc::clone(&current_line), fragment.clone()));
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