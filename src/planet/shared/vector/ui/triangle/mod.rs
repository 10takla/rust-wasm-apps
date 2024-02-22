mod impls;
#[cfg(test)]
mod tests;

use crate::planet::shared::vector::Vector;
pub struct Triangle{
    pub a: Vector,
    pub c: Vector,
    pub b: Vector,
}

impl Triangle {
    pub fn get_angle_by_point(&self, target: usize) -> f64 {
        let other_ids: Vec<usize> = (0..=2).filter(|&i| i != target).collect();
        let verts = [self.a, self.b, self.c];
        let (a, b, c) = (verts[other_ids[0]], verts[target], verts[other_ids[1]]);
        let ab = a - b;
        let bc = c - b;
        (ab.scalar(&bc) / (ab.radius() * bc.radius()))
            .acos()
            .to_degrees()
    }

    pub fn get_angles(&self) -> [f64; 3] {
        let mut tmp = [0.0; 3];
        for (i, v) in [self.a, self.b, self.c].iter().enumerate() {
            tmp[i] = self.get_angle_by_point(i);
        }
        tmp
    }
}

impl Triangle {
    pub fn get_center(&self) -> Vector {
        let (a, b, c) = (self.a, self.b, self.c);
        let (c_ab, c_bc) = ((a + b) / 2.0, (b + c) / 2.0);
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
        let x = (e_ab * v_bc[1] - v_ab[1] * e_bc) / (v_ab[0]  * v_bc[1] - v_ab[1] * v_bc[0]);
        let y = (e_bc - v_bc[0] * x) / v_bc[1];
        [x, y].into()
    }
}
