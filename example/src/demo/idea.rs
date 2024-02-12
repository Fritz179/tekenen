// It target passed by draw
// How can update change position/size?
// TransformData, Option(TV)?

use std::{rc::Rc, cell::RefCell};

use tekenen::{platform::{Event, Platform, IntervalDecision, PlatformTrait}, Tekenen, ui::{Element, Contain}};

use super::Demo;

pub struct DemoIdea {
    tek: Tekenen,
    // manager: DemoManger,
}

impl DemoIdea {
    pub fn new() -> Self {
        let tek = Tekenen::new(800, 600);
        // let manager = todo!();

        Self {
            tek,
            // manager
        }
    }
}

// impl Demo for DemoIdea {
//     fn draw(&mut self, window: &mut Platform) {
//         self.manager.update();
//         self.manager.draw();

//         window.display_pixels(self.tek.get_pixels());
//     }

//     fn update(&mut self, event: &Event) -> IntervalDecision {
//         self.manager.event(event);

//         IntervalDecision::Repeat
//     }
// }

// struct DemoManger {
//     left: Element,
//     right: Element,
// }

// impl Contain for DemoManger {
//     fn get_children(&mut self) -> &[&mut dyn Element] {
//         &[
//             &mut self.left,
//             &mut self.right,
//         ]
//     }
// }

// What if Container calls self.tv.ui(self.sub_container) & self.sub_element

// How to get sizes? On creation?
// get_tv can set size?
// Containers can test if child is container and downcast
// Event => Resize

// Would be easier if there is no difference between Element & Container
// Container as Wrapper? 
// get_tv(&mut self) -> Option(&Rc<RefCell<dyn Draw>>)
// What if target is only rc and not RefCell, target inner mutability? tekenen inner mutable?
// get_tv determine if resize event is needed
// update, event, draw => After setting 