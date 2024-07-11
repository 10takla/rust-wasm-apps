mod impls;
pub mod svg;
#[cfg(test)]
mod tests;
pub mod ui;

use super::super::Angle;
use crate::planet::shared::traits::Projection;
use crate::{
    planet::shared::{
        point::{DefaultMeasureValue, DEFAULT_MEASURE},
        traits::As,
        vector::{
            ui::{
                circle::Circle,
                line::{Line, LineType},
            },
            Number, Vector, VectorType,
        },
    },
    traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    },
};
use macros::Iterator;
use num::integer::sqrt;
use serde::Serialize;
use std::rc::Rc;

#[derive(Clone, Iterator)]
pub struct Triangle<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> {
    // в center угла Central координата, по бокам соседние по часовой стрелке
    pub cab: Rc<Angle<T, N>>,
    pub abc: Rc<Angle<T, N>>,
    pub bca: Rc<Angle<T, N>>,
}

pub type TriangleType<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> = Rc<Triangle<T, N>>;

impl<T: Number, const N: usize> Triangle<T, N> {
    pub fn area(&self) -> f64 {
        let [a, b, c] = self.to::<[LineType<T, N>; 3]>().map(|line| line.length());

        let s = (a + b + c) / 2.as_::<T>();
        (s * (s - a) * (s - b) * (s - c)).as_::<f64>().sqrt()
    }

    pub fn get_heights(&self) -> [T; 3] {
        let k = self.to::<[VectorType<T, N>; 3]>();

        let h = |a, b, c| {
            let [a, b, c] = [a, b, c].map(|v| v * v);

            let y = (b - c);
            let t = (b + c - (a + y * y / a) / 2.as_::<T>()) / 2.as_::<T>();

            t
        };

        k.iter()
            .map(|a| {
                let [b, c] = k
                    .iter()
                    .filter(|&v| !Rc::ptr_eq(v, a))
                    .collect::<Vec<&VectorType<T, N>>>()
                    .try_into()
                    .unwrap();

                h(*a.clone(), *b.clone(), *c.clone())
                    .radius()
                    .as_::<f64>()
                    .sqrt()
                    .as_()
            })
            .collect::<Vec<T>>()
            .try_into()
            .unwrap()
    }
}

impl<T: Number, const N: usize> Triangle<T, N> {
    /// Решение системы уравнения для центра пересечения серединных перпендикуляров двух ребер
    // {
    //     (b.x-a.x)(x-c_ab.x) + (b.y-a.y)(y-c_ab.y) = 0
    //     (c.x-b.x)(x-c_bc.x) + (c.y-b.y)(y-c_bc.y) = 0
    // }
    // {
    //     (b.x*x - b.x*c_ab.x - a.x*x + a.x*c_ab.x) + (b.y*y - b.y*c_ab.y - a.y*y + a.y*c_ab.y) = 0
    //     (c.x*x - c.x*c_bc.x - b.x*x + b.x*c_bc.x) + (c.y*y - c.y*c_bc.y - b.y*y + b.y*c_bc.y) = 0
    // }
    // {
    //     (b.x*x - a.x*x) + (b.y*y - a.y*y) = (b.x*c_ab.x - a.x*c_ab.x) + (b.y*c_ab.y - a.y*c_ab.y)
    //     (c.x*x - b.x*x) + (c.y*y - b.y*y) = (c.x*c_bc.x - b.x*c_bc.x) + (c.y*c_bc.y - b.y*c_bc.y)
    // }
    // [
    //     Eab = b.x*c_ab.x - a.x*c_ab.x + b.y*c_ab.y - a.y*c_ab.y
    //     Ebc = c.x*c_bc.x - b.x*c_bc.x + c.y*c_bc.y - b.y*c_bc.y
    // ]
    // */
    /*
    {
        (b.x - a.x)x + (b.y - a.y)y = Eab
        (c.x - b.x)x + (c.y - b.y)y = Ebc
    }
    [
        Vab = [(b.x - a.x), (b.y - a.y)]
        Vbc = [(c.x - b.x), (c.y - b.y)]
    ]
    */
    /*
    {
        Vab.x*x + Vab.y*y = Eab
        Vbc.x*x + Vbc.y*y = Ebc
    }
    {
        Vab.x*x = Eab - Vab.y*y
        y = (Ebc - Vbc.x*x) / Vbc.y
    }
    [
        Vab.x*x = Eab - Vab.y*((Ebc - Vbc.x*x)/Vbc.y)
        Vab.x*x = Eab - (Vab.y*Ebc - Vab.y*Vbc.x*x)/Vbc.y
        Vab.x*x = (Eab*Vbc.y - Vab.y*Ebc + Vab.y*Vbc.x*x)/Vbc.y
        Vbc.y*Vab.x*x = Eab*Vbc.y - Vab.y*Ebc + Vab.y*Vbc.x*x
        Vbc.y*Vab.x*x - Vab.y*Vbc.x*x = Eab*Vbc.y - Vab.y*Ebc
        (Vbc.y*Vab.x - Vab.y*Vbc.x)*x = Eab*Vbc.y - Vab.y*Ebc
        x = (Eab*Vbc.y - Vab.y*Ebc) / (Vab.x*Vbc.y - Vbc.x*Vab.y)
    ]
    {
        x = (Eab - Vab.y*y) / Vab.x
        Vbc.y*y = Ebc - Vbc.x*x
        [
            ...
            y = (Ebc*Vab.x - Vbc.x*Eab) / (Vab.x*Vbc.y - Vbc.x*Vab.y)
        ]
    }
    */
    pub fn get_circle(&self) -> Circle<T, N> {
        if N < 2 {
            panic!("описанная окружность находиться от 2-x и более мерного пространства.\nНа данный момент N == {N}")
        }

        let [a, b, c]: [Vector<f64, N>; 3] = [
            (&self.abc.ba.b).as_(),
            (&self.abc.ba.a).as_(),
            (&self.abc.bc.b).as_(),
        ];

        if (a == b || b == c || a == c)
            || (0..a.len()).into_iter().any(|i| {
                [[a, b], [b, c], [c, a]]
                    .into_iter()
                    .all(|[v1, v2]| v1[i] == v2[i])
            })
        {
            let mut t = [a, b, c].to_vec();
            t.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));

            return Circle {
                center: Rc::new(((t[2] - t[0]) / 2.0).as_()),
                point: Rc::new(a.as_()),
            };
        }

        let [c_ab, c_bc]: [Vector<f64, N>; 2] = [(a + b) / 2.0, (b + c) / 2.0];

        let v_ab = (b - a);
        let v_bc = (c - b);

        let mut center = [0.; N];
        center[0] = {
            let e_ab = (b * c_ab).filter::<2>([0, 1]).sum() - (a * c_ab).filter::<2>([0, 1]).sum();
            let e_bc = (c * c_bc).filter::<2>([0, 1]).sum() - (b * c_bc).filter::<2>([0, 1]).sum();
            let v_ab = v_ab.filter::<2>([0, 1]);
            let v_bc = v_bc.filter::<2>([0, 1]);

            (e_ab * v_bc[1] - v_ab[1] * e_bc) / (v_ab[0] * v_bc[1] - v_bc[0] * v_ab[1])
        };

        for i in (0..N - 1) {
            let axises = [i, i + 1];

            center[i + 1] = {
                let e_ab =
                    (b * c_ab).filter::<2>(axises).sum() - (a * c_ab).filter::<2>(axises).sum();
                let e_bc =
                    (c * c_bc).filter::<2>(axises).sum() - (b * c_bc).filter::<2>(axises).sum();
                let v_ab = v_ab.filter::<2>(axises);
                let v_bc = v_bc.filter::<2>(axises);

                (e_bc * v_ab[0] - v_bc[0] * e_ab) / (v_bc[1] * v_ab[0] - v_ab[1] * v_bc[0])
            };
        }

        Circle {
            point: Rc::new(self.abc.ba.a.as_()),
            center: Rc::new(center.to::<Vector<f64, N>>().as_()),
        }
    }
}
