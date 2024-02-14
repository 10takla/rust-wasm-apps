use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

use super::Point;

// From
impl From<[f64; 2]> for Point {
    fn from(value: [f64; 2]) -> Self {
        Self(value)
    }
}

// использовать непосредственно поле кортежа
impl Deref for Point {
    type Target = [f64; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// сравнение
impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        for (i, m) in self.0.iter().enumerate() {
            match m.partial_cmp(&other.0[i]).unwrap() {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => continue,
            }
        }
        return Ordering::Equal;
    }
}
