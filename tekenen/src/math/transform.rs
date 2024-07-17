use super::Vec2;

// #[derive(Debug, Clone, Copy)]
// pub struct SurfaceTransformation {
//     translation: Vec2,
//     scale: f32,
// }

// impl SurfaceTransformation {
//     pub fn new() -> Self {
//         Self {
//             translation: Vec2::new(0, 0),
//             scale: 1.0,
//         }
//     }

//     pub fn translate_screen(&mut self, translation: Vec2) {
//         self.translation += translation;
//     }

//     pub fn transalte_world(&mut self, translation: Vec2) {
//         self.translation += translation * self.scale;
//     }

//     pub fn scale_screen(&mut self, from: Vec2, scale: f32) {
//         self.translation += from * (1.0 - scale);
//         self.scale *= scale;
//     }

//     pub fn scale_world(&mut self, from: Vec2, scale: f32) {
//         self.translation += from * (1.0 - scale) * self.scale;
//         self.scale *= scale;
//     }

//     pub fn world_point_to_screen(&self, point: Vec2) -> Vec2 {
//         point * self.scale + self.translation
//     }

//     pub fn screen_point_to_world(&self, point: Vec2) -> Vec2 {
//         (point - self.translation) / self.scale
//     }

//     pub fn world_length_to_screen(&self, length: f32) -> f32 {
//         length * self.scale
//     }

//     pub fn screen_length_to_world(&self, length: f32) -> f32 {
//         length / self.scale
//     }
// }

pub trait Transform {
    fn translate(&mut self, translation: Vec2);
    fn scale(&mut self, scale: f32);
}