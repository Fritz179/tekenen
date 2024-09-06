use std::{cell::RefCell, rc::Rc};

use tekenen::{colors, fui::{button::Button, div::Div, slider::Slider, text::Text, FUI}, platform::{Event, IntervalDecision, KeyDownEvent, MouseKey, Platform, PlatformTrait}, printer::Printer, shapes::{rect::Rect, Shape}, DrawableSurface, Surface, SurfaceView};

use super::Demo;

pub struct InteractionsDemo {
    tekenen: SurfaceView,
    fui: FUI,
    text: Rc<Text>,

    canvas: SurfaceView,
    shapes: Vec<Rc<RefCell<dyn Shape>>>,
    creator: Option<Rc<RefCell<Rect>>>
}

impl InteractionsDemo {
    pub fn new() -> Self {
        let text = Text::new("Hello, world!");

        // Button
        let mut clicks: i32 = 0;
        let button_text = Text::new("Clicks: 0");
        let button_text_clone = button_text.clone();
        let button = Button::new(move || {
            clicks += 1;
            button_text_clone.set_text(format!("Clicks: {}", clicks));
        });

        // Slider
        let slider_text = Text::new("Value: 50%");
        let slider_text_clone = slider_text.clone();
        let slider = Slider::new(move |value| {
            slider_text_clone.set_text(format!("Value: {}%", (value * 100.0) as i32));
        });

        let fui = FUI::new(Div::new(vec![
            text.clone(), 
            Text::new("Second line?"),
            button,
            button_text,
            slider,
            slider_text,
        ]));

        let tekenen = SurfaceView::new(800, 600, Surface::new(800, 600).into());
        let canvas = tekenen.tee();
        canvas.clip(Rect::new(0, 300, 800, 300));

        Self {
            tekenen,
            fui,
            text,
            canvas,
            shapes: vec![Rc::new(RefCell::new(Rect::new(0, 0, 100, 100)))],
            creator: None
        }
    }
}

impl Demo for InteractionsDemo {
    fn update(&mut self, mut event: Event) -> IntervalDecision {
        self.fui.event(event);

        self.canvas.handle_pan_and_zoom(event);

        if let Event::KeyDown(KeyDownEvent { char: Some(key), .. }) = event {
            match key {
                'd' => println!("{:?}", Printer::new(&self.fui)),
                's' => println!("{}", Printer::new(&self.fui)),
                _ => { }
            }
        }

        self.canvas.screen_to_world(&mut event);

        match event {
            Event::MouseDown{ x, y, key: MouseKey::Left } => {
                let shape = Rc::new(RefCell::new(Rect::new(x, y, 1, 1)));
                self.creator = Some(shape.clone());
                self.shapes.push(shape);
            },
            Event::MouseMove{ x, y, .. } => {
                if let Some(rect) = self.creator.as_ref() {
                    let mut rect = rect.borrow_mut();
                    rect.size.x = x - rect.position.x;
                    rect.size.y = y - rect.position.y;
                }
            },
            Event::MouseUp{ .. }  => {
                self.creator = None;
            },
            _ => { }
        }

        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut Platform, tick: i32) {
        let ctx = &self.tekenen;

        self.text.set_text(format!("Hello, world! {}", tick));

        ctx.background(colors::FRITZ_GRAY);
        self.fui.render(ctx);

        self.canvas.background(colors::BLACK);
        self.canvas.fill_color(colors::GREEN);
        
        for shape in &self.shapes {
            let shape = shape.borrow_mut();
            let mut clone = shape.dyn_clone();

            self.canvas.dyn_shape(clone.as_mut())
        }

        window.display_surface(ctx.get_surface());
    }
}