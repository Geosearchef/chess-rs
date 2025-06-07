use crate::chess::board::{BOARD_SIZE_X, BOARD_SIZE_Y};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vector(pub i8, pub i8);

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<i8> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i8) -> Self::Output {
        Vector(self.0 * rhs, self.1  * rhs)
    }
}

impl Vector {
    pub fn is_on_board(&self) -> bool {
        self.0 < BOARD_SIZE_X && self.1 < BOARD_SIZE_Y && self.0 >= 0 && self.1 >= 0
    }
}