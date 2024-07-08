use std::cell::{Ref, RefCell};

use std::rc::Rc;

use tekenen::shapes::rect::Rect;
use tekenen::ui::elements::{Div, Text, Element};
use tekenen::ui::style::{CSSDisplay, CSSFlexDirection, CSSPadding, Context};
use tekenen::{colors, Draw, Tekenen};
use tekenen::platform::{PlatformTrait, IntervalDecision, Event, KeyDownEvent};

pub struct TestDemo {
    tek: Tekenen,
    div: Rc<RefCell<Div>>,
}

// <div style="background-color: yellow; max-width: 350px; ">
//     Outside text
//     <p style="background-color: lightgreen;">
//         Paragraph!
//     </p>

//     <div style="background-color: lightgray; width: 300px; padding-left: 50%; color: red">
//         Oh no!
//         I'm outside the parents space!
//         WhatIfIHaveAVeryLongWordWhenWillIWrap
//     </div>
//     <p>
//         I'm an outside text and I'm going to wrap when needed at soft wraps
//     </p>
// </div>

impl TestDemo {
    pub fn new() -> Self {
        let div = Div::new_vertical_flex(vec![
            Div::new_fn(vec![
                Text::new_frag("Outside text"),
                Text::new_fn("Paragraph!", | el | {
                    el.style.background_color.set(colors::css::LIGHTGREEN);
                }),
                Div::new_fn(vec![
                    Text::new_frag("Oh no! I'm outside the parents space! WhatIfIHaveAVeryLongWordWhenWillIWrap"),
                ], | el | {
                    el.style.display = CSSDisplay::Flex;
                    el.style.flex_direction = CSSFlexDirection::Column;
                    el.style.background_color.set(colors::css::LIGHTGRAY);
                    el.style.width.set(300.into());
                    el.style.padding.left.set_percent(0.5.into());
                }),
                Text::new_frag("I'm an outside text and I'm going to wrap when needed at soft wraps"),
            ], | el | {
                el.style.display = CSSDisplay::Flex;
                el.style.flex_direction = CSSFlexDirection::Column;
                el.style.background_color.set(colors::css::YELLOW);
                el.style.max_width.set(350.into());
            })
        ]);

        Self {
            tek: Tekenen::new(800, 600),
            div
        }
    }
}

impl super::Demo for TestDemo {
    fn update(&mut self, event: &Event) -> tekenen::platform::IntervalDecision {
        match event {
            Event::Quit => {
                return IntervalDecision::Stop
            },
            Event::KeyDown(KeyDownEvent { char: Some(char), .. }) => {
                println!("{char}")
            },
            _ => { }
        };

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut tekenen::platform::Platform) {
        println!();
        println!("Drawing");

        // Clear canvas
        let tekenen = &mut self.tek;
        tekenen.set_translation(0, 0);
        tekenen.background(colors::FRITZ_GRAY);

        // Get the Paint Tree
        let context = Context {
            containing_block: Rect::new(0, 0, 800, 600)
        };
        let div: Ref<'_, dyn Element> = self.div.borrow();

        let layout = div.get_style().get_layout(Rc::clone(&self.div) as Rc<RefCell<dyn Element>>, Rect::new(0, 0, 800, 600), context);
        let painter = div.get_style().get_painter(Rc::clone(&self.div) as Rc<RefCell<dyn Element>>, Rect::new(0, 0, 800, 600), context);

        // Draw the Paint Tree
        painter.paint(tekenen);

        // Display the pixels
        window.display_pixels(tekenen.get_pixels());
    }
}