use crate::derive_deref;

use super::super::Triangle;

pub struct Plane([Triangle; 2]);
derive_deref!(Plane, 0, [Triangle; 2]);

impl Plane {
    // pub fn reverse_tries(&self) {
    //     self[0].
    // }
}

