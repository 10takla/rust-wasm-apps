#[cfg(test)]
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::planet::point_distribution::DefaultMeasureValue;

use super::Point;




#[wasm_bindgen]
pub struct Circle {
    a: Point,
    b: Point,
    c: Point,
}

#[wasm_bindgen]
impl Circle {
    fn get_center(&self) -> Point {
        #[derive(Debug)]
        struct P {
            x: DefaultMeasureValue,
            y: DefaultMeasureValue,
        }
        
        impl P {
            fn new(x: DefaultMeasureValue, y: DefaultMeasureValue) -> Self {
                P{x,y}
            }
        }
        let ([x_a, y_a], [x_b, y_b], [x_c, y_c]) = (self.a, self.b, self.c);
        let (a, b, c) = (P::new(x_a, y_a), P::new(x_b, y_b), P::new(x_c, y_c));
        dbg!(&a, &b, &c);

        /*
        (a.x-o.x)^2 + (a.y-o.y)^2 = r^2
        (b.x-o.x)^2 + (b.y-o.y)^2 = r^2
        (c.x-o.x)^2 + (c.y-o.y)^2 = r^2

        (a.x-o.x)^2 + (a.y-o.y)^2 = (b.x-o.x)^2 + (b.y-o.y)^2
        (a.x-o.x)^2 + (a.y-o.y)^2 = (c.x-o.x)^2 + (c.y-o.y)^2
        
        (a.x^2 -2*a.x*o.x + o.x^2) + (a.y^2 -2*a.y*o.y + o.y^2) = (b.x^2 -2*b.x*o.x + o.x^2) + (b.y^2 -2*b.y*o.y + o.y^2)
        (a.x^2 + a.y^2) - 2(a.x*o.x + a.y*o.y) + 2o.x^2 = (b.x^2 + b.y^2) - 2(b.x*o.x + b.y*o.y) + 2o.x^2
        (a.x^2 + a.y^2) - 2(a.x*o.x + a.y*o.y) = (b.x^2 + b.y^2) - 2(b.x*o.x + b.y*o.y)
        (b.x*o.x + b.y*o.y) - (a.x*o.x + a.y*o.y) = ((b.x^2 + b.y^2) - (a.x^2 + a.y^2))/2
        b.x*o.x + b.y*o.y - a.x*o.x - a.y*o.y = ((b.x^2 - a.x^2) + (b.y^2 - a.y^2))/2
        2(b.x-a.x)o.x + 2(b.y-a.y)o.y = (b.x^2 - a.x^2) + (b.y^2 - a.y^2)
        
        Xba = 2(b.x-a.x) Yba = 2(b.y-a.y)
        PXba = (b.x^2 - a.x^2) PYba = (b.y^2 - a.y^2)
        PXca = (c.x^2 - a.x^2) PYca = (c.y^2 - a.y^2)
        */
        let (x_ba, y_ba) = (2.0 *(b.x-a.x), 2.0 *(b.y-a.y));
        let (x_ca, y_ca) = (2.0 *(c.x-a.x), 2.0 *(c.y-a.y));
        let (p_x_ba, p_y_ba) = ((b.x.powf(2.0) - a.x.powf(2.0)), (b.y.powf(2.0) - a.y.powf(2.0)));
        let (p_x_ca, p_y_ca) = ((c.x.powf(2.0) - a.x.powf(2.0)), (c.y.powf(2.0) - a.y.powf(2.0)));
        /*      
        Xba * o.x + Yba * o.y = PXba + PYba
        Xca * o.x + Yca * o.y = PXca + PYca
        
        o.x = ((PXba + PYba) - Yba * o.y)/Xba
        o.y = ((PXca + PYca) - Xca * o.x)/Yca
        
        o.y = ((PXca + PYca) - Xca * ((PXba + PYba) - Yba * o.y) /Xba )/Yca

        Tca = (PXca + PYca) Tba = (PXba + PYba)
        */
        let (t_ca, t_ba) = (p_x_ca + p_y_ca, p_x_ba + p_y_ba);
        /*
        o.y = (Tca + (-Xca * (Tba - Yba*o.y) /Xba) )/Yca
        o.y = (Tca + (-Xca * Tba + Xca*Yba*o.y) /Xba) )/Yca
        o.y = (Xba*Tca - Xca*Tba + Xca*Yba*o.y)*Yca / Xba
        
        K1 = Xba*Tca  K2 = Xca*Tba
        */
        let (k1, k2) = (x_ba*t_ca, x_ca*t_ba);
        /*
        o.y = (K1*Yca - K2*Yca+ Xca*Yba*o.y*Yca) / Xba
        o.y = (K1*Yca - K2*Yca)/Xba  +  (Xca*Yba*Yca/Xba)*o.y

        L1 = (Xca*Yba*Yca/Xba) L2 = (K1*Yca - K2*Yca)/Xba
        */
        let (l1, l2) = (x_ca*y_ba*y_ca/x_ba, k1*y_ca - k2*y_ca);
        /*
        o.y - L1*o.y = L2
        o.y = L2 / (1 - L1)
        o.x = (Tba - Yba*o.y)/Xba
        */
        let o_y = l2/(1.0 - l1);
        let o_x = (t_ba - y_ba*o_y)/x_ba;
        dbg!(o_x, o_y);
        [o_x, o_y]
    }
}

