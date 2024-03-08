pub type DefaultMeasureValue = f64;
pub type Point<T = DefaultMeasureValue, const N: usize = 2> = [T; N];
pub type Points<T = DefaultMeasureValue, const N: usize = 2> = Vec<Point<T, N>>;