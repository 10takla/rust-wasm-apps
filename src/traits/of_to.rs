type Point = i32;
#[derive(Copy, Clone)]
struct Vector(Point);

pub trait Of<F>: To {
    fn of(value: F) -> Self;
}

pub trait To {
    fn to<I: Of<Self>>(self) -> I
    where
        Self: Sized;
}

impl<F> To for F {
    fn to<I: Of<F>>(self) -> I {
        I::of(self)
    }
}

impl Of<Vector> for Point {
    fn of(value: Vector) -> Self {
        value.0
    }
}