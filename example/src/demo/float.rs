use std::rc::Rc;

use tekenen::shapes::rect::Rect;
use tekenen::ui::elements::{BlockBlockFormattingContext, FormattingContext, Div, DomElement, LayoutNode, P};

use tekenen::ui::style::{CSSDisplayShorthand, FormattingInfo};
use tekenen::{colors, Draw, Tekenen};
use tekenen::platform::{PlatformTrait, IntervalDecision, Event, KeyDownEvent};

pub struct FloatDemo {
    tek: Tekenen,
    div: Rc<Div>,
    print_layout: bool,
    print_painter: bool,
}

impl FloatDemo {
    pub fn new() -> Self {
        // Text
        let div = Div::new_fn(vec![
            P::new_fn("hello there, this text should wrap multiple times, how nice!", |p| {
                p.style.borrow_mut().background_color.set(colors::RED);
            }),
            P::new_fn("world!", |p| {
                p.style.borrow_mut().background_color.set(colors::GREEN);
            }),
            P::new("worldddddddddddddddddddd!"),
            P::new("world!!!"),
            P::new("world!"),
        ], |div| {
            div.style.borrow_mut().background_color.set(colors::MAROON);
            div.style.borrow_mut().width.set(250.into());
        });

        // Flex
        // let div = Div::new_fn(vec![
        //     P::new_fn("hello there, this text should wrap multiple times, how nice!", |p| {
        //         p.style.borrow_mut().background_color.set(colors::RED);
        //     }),
        //     P::new_fn("world!", |p| {
        //         p.style.borrow_mut().background_color.set(colors::GREEN);
        //     }),
        //     P::new("worldddddddddddddddddddd!"),
        //     P::new("world!!!"),
        //     P::new("world!"),
        // ], |div| {
        //     div.style.borrow_mut().background_color.set(colors::MAROON);
        //     div.style.borrow_mut().display = CSSDisplayShorthand::Flex.into();
        // });


        Self {
            tek: Tekenen::new(800, 600),
            div,
            print_layout: false,
            print_painter: false,
        }
    }
}

impl super::Demo for FloatDemo {
    fn update(&mut self, event: &Event) -> tekenen::platform::IntervalDecision {
        match event {
            Event::Quit => {
                return IntervalDecision::Stop
            },
            Event::KeyDown(KeyDownEvent { char: Some(char), .. }) => {
                match char {
                    'd' => println!("{}", self.div.clone() as Rc<dyn DomElement>),
                    'l' => self.print_layout = true,
                    'p' => self.print_painter = true,
                    _ => { }
                }
            },
            _ => { }
        };

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut tekenen::platform::Platform) {

        // 1. Clear canvas
        let tekenen = &mut self.tek;
        tekenen.set_translation(0, 0);
        tekenen.background(colors::FRITZ_GRAY);

        // 2. Generate Layout Box Tree
        let layout = LayoutNode::new(Some(self.div.clone()));
        
        if self.print_layout {
            println!("{layout}")
        }

        // 3. Do Layouting and get Paint Tree
        let context = BlockBlockFormattingContext::new();
        let info = FormattingInfo {
            containing_block: Rect::new(0, 0, 800, 600),
        };

        let painter = context.run(&layout, &info).unwrap();

        if self.print_painter {
            println!("{painter}");
        }

        // 4. Draw the Paint Tree
        painter.paint(tekenen);

        // 5. Display the pixels
        window.display_pixels(tekenen.get_pixels());

        
        self.print_layout = false;
        self.print_painter = false;
    }
}