use crate::planet::shared::{point::{DefaultMeasureValue, Point}, vector::Vector};

#[cfg(test)]
mod tests;


pub struct Circumference {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Circumference {
    pub fn get_center(&self) -> Vector {
        #[derive(Debug, Clone, Copy)]
        struct P {
            x: DefaultMeasureValue,
            y: DefaultMeasureValue,
        }

        impl P {
            fn new(p: Point) -> Self {
                Self { x: p[0], y: p[1] }
            }
        }

        macro_rules! mul {
            ($constructor:expr, $($arg:ident),*) => {{
                    $(
                        let $arg  = $constructor($arg);
                    )*
            }};
        }
        let (a, b, c) = (self.a, self.b, self.c);
        mul!(P::new, a, b, c);

        let (a, b, c) = (P::new(self.a), P::new(self.b), P::new(self.c));

        let c_ab = {
            let r = Vector(self.a) + Vector(self.b);
            P {
                x: r.0[0] / 2.0,
                y: r.0[1] / 2.0,
            }
        };
        let c_bc = {
            let r = Vector(self.b) + Vector(self.c);
            P {
                x: r.0[0] / 2.0,
                y: r.0[1] / 2.0,
            }
        };
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
        let e_ab = b.x * c_ab.x - a.x * c_ab.x + b.y * c_ab.y - a.y * c_ab.y;
        let e_bc = c.x * c_bc.x - b.x * c_bc.x + c.y * c_bc.y - b.y * c_bc.y;
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
        let v_ab = P {
            x: b.x - a.x,
            y: b.y - a.y,
        };
        let v_bc = P {
            x: c.x - b.x,
            y: c.y - b.y,
        };
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
        // println!("{:?}", (v_bc.y * v_ab.x - v_ab.y * v_bc.x));
        let x = (e_ab * v_bc.y - v_ab.y * e_bc) / (v_bc.y * v_ab.x - v_ab.y * v_bc.x);
        // println!("{:?}", x);
        let y = (e_bc - v_bc.x * x) / v_bc.y;
        // dbg!(e_bc, v_bc, x);
        // dbg!();
        // dbg!(v_bc.x * x, e_bc - v_bc.x * x, v_bc.y, y);
        Vector([x, y])
    }
}
