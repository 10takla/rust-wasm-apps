use crate::derive_deref;

use super::super::Triangle;

pub struct Plane<T>([Triangle<T>; 2]);
derive_deref!(Plane, 0, [Triangle<T>; 2], T);

impl<T> Plane<T> {
    // pub fn reverse_tries(&self) {
    //     self[0].
    // }
}

