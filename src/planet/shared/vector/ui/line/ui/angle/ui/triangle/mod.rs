mod impls;
#[cfg(test)]
mod tests;
pub mod ui;

use std::rc::Rc;
use super::super::Angle;
use crate::{planet::shared::{
    point::DefaultMeasureValue,
    vector::{ui::circle::Circle, Number, Vector},
}, traits::of_to::To};
use crate::traits::of_to::Of;

#[derive(Debug, Clone)]
pub struct Triangle<T = DefaultMeasureValue, const N: usize = 2> {
    // в center угла Central координата, по бокам соседние по часовой стрелке
    pub cab: Rc<Angle<T, N>>,
    pub abc: Rc<Angle<T, N>>,
    pub bca: Rc<Angle<T, N>>,
}

impl<T: Number> Triangle<T> {
    pub fn get_circle(&self) -> Circle<T, 2> {
        let [a, b, c]: [Vector; 3] = [
            (&self.abc.ba.b).as_(),
            (&self.abc.ba.a).as_(),
            (&self.abc.bc.b).as_(),
        ];
        let [c_ab, c_bc]: [Vector; 2] = [(a + b) / 2.0, (b + c) / 2.0];
        /*
        {
            (b.x-a.x)(x-c_ab.x) + (b.y-a.y)(y-c_ab.y) = 0
            (c.x-b.x)(x-c_bc.x) + (c.y-b.y)(y-c_bc.y) = 0
        }
        {
            b.x*x - b.x*c_ab.x - a.x*x + a.x*c_ab.x + b.y*y - b.y*c_ab.y - a.y*y + a.y*c_ab.y = 0
            c.x*x - c.x*c_bc.x - b.x*x + b.x*c_bc.x + c.y*y - c.y*c_bc.y - b.y*y + b.y*c_bc.y = 0
        }
        {
            b.x*x - a.x*x + b.y*y - a.y*y = b.x*c_ab.x - a.x*c_ab.x + b.y*c_ab.y - a.y*c_ab.y
            c.x*x - b.x*x + c.y*y - b.y*y = c.x*c_bc.x - b.x*c_bc.x + c.y*c_bc.y - b.y*c_bc.y
        }
        [
            Eab = b.x*c_ab.x - a.x*c_ab.x + b.y*c_ab.y - a.y*c_ab.y
            Ebc = c.x*c_bc.x - b.x*c_bc.x + c.y*c_bc.y - b.y*c_bc.y
        ]
        */

        let e_ab = (b * c_ab).sum() - (a * c_ab).sum();
        let e_bc = (c * c_bc).sum() - (b * c_bc).sum();
        /*
        {
            (b.x - a.x)x + (b.y - a.y)y = Eab
            (c.x - b.x)x + (c.y - b.y)y = Ebc
        }
        [
            Vab = (b.x - a.x); (b.y - a.y)
            Vbc = (c.x - b.x); (c.y - b.y)
        ]
        */
        let v_ab = b - a;
        let v_bc = c - b;
        /*
        {
            Vab.x*x + Vab.y*y = Eab
            Vbc.x*x + Vbc.y*y = Ebc
        }
        {
            Vab.x*x = Eab - Vab.y*y
            y = (Ebc - Vbc.x*x)/Vbc.y
        }
        [
            Vab.x*x = Eab - Vab.y*((Ebc - Vbc.x*x)/Vbc.y)
            Vab.x*x = Eab - (Vab.y*Ebc - Vab.y*Vbc.x*x)/Vbc.y
            Vab.x*x = (Eab*Vbc.y - Vab.y*Ebc + Vab.y*Vbc.x*x)/Vbc.y
            Vbc.y*Vab.x*x = Eab*Vbc.y - Vab.y*Ebc + Vab.y*Vbc.x*x
            Vbc.y*Vab.x*x - Vab.y*Vbc.x*x = Eab*Vbc.y - Vab.y*Ebc
            (Vbc.y*Vab.x - Vab.y*Vbc.x)*x = Eab*Vbc.y - Vab.y*Ebc
            x = (Eab*Vbc.y - Vab.y*Ebc) / (Vbc.y*Vab.x - Vab.y*Vbc.x)
        ]
        */
        let x = (e_ab * v_bc[1] - v_ab[1] * e_bc) / (v_ab[0] * v_bc[1] - v_ab[1] * v_bc[0]);
        let mut y = (e_bc - v_bc[0] * x) / v_bc[1];
        if f64::is_nan(y) {
            y = dbg!(Triangle::of([b, a, c]).get_circle().center[1]);
        }

        Circle {
            point: Rc::new(self.abc.ba.a.as_()),
            center: Rc::new([x, y].to::<Vector>().as_()),
        }
    }
}
