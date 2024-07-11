pub type DefaultMeasureValue = f64;
pub const DEFAULT_MEASURE: usize = 2;
pub type Point<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> = [T; N];
pub type Points<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> = Vec<Point<T, N>>;