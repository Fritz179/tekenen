use tekenen::platform::{Event, Platform, IntervalDecision, PlatformTrait};
use tekenen::{Tekenen, colors};
use tekenen::ui::{Container, Text};

pub struct TextDemo {
    tek: Tekenen
}

impl TextDemo {
    pub fn new() -> Self {
        Self {
            tek: Tekenen::new(800, 600)
        }
    }
}

impl super::Demo for TextDemo {
    fn update(&mut self, event: Event) -> IntervalDecision {
        IntervalDecision::Repeat
    }

    fn draw(&mut self, window: &mut Platform) {
        let tek = &mut self.tek;

        tek.background(colors::GRAY);

        
        
        tek.ui(&mut Container::vertical(vec![
            Text::new("This is a Section!"),
            Container::horizontal(vec![
                Text::new("Line 1:"),
                Text::new("<x>")
            ]),//.justify(justify::space_beetwen),
            Container::horizontal(vec![
                    Text::new("Line 1:"),
                    Text::new("<y>")
                ])
                ,//.justify(justify::space_beetwen),
                // .border(border::new()::bottom(unit::pixels::new(20))),
            Text::new("This is anorher Section!"),
            Container::horizontal(vec![
                Text::new("A:"),
                Text::new("<z>")
            ]),//.justify(justify::space_beetwen),
            Container::horizontal(vec![
                    Text::new("A very very very very very very long Line:"),
                    Text::new("<w>")
                ])
                ,//.justify(justify::space_beetwen)
        ]));

        window.display_pixels(tek.get_pixels());
    }
}