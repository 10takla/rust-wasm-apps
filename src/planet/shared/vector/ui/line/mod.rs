#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub a: Vector,
    pub b: Vector,
}

impl Line {
    fn get_vector(&self) -> Vector {
        self.b - self.a
    }
}