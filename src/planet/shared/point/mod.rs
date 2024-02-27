pub type DefaultMeasureValue = f64;
pub type Point<T = DefaultMeasureValue> = [T; 2];
pub type Points = Vec<Point>;


pub trait ToPoints {
    fn to_points<const N: usize>(&self) -> [Point; N];
}