use std::ops::Mul;

use super::Vec2;

pub struct Mat<const COLS: usize, const ROWS: usize> {
    pub matrix: [[i32; ROWS]; COLS]
}

pub type TransfromMatrix = Mat<2, 3>;

impl Mul<Vec2> for TransfromMatrix {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: rhs.x * self.matrix[0][0] + rhs.y * self.matrix[0][1] + self.matrix[0][2],
            y: rhs.x * self.matrix[1][0] + rhs.y * self.matrix[1][1] + self.matrix[1][2],
        }
    }
}