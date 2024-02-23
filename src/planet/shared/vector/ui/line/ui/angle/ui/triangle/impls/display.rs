use core::fmt;
use std::fmt::{Display, Formatter};
use crate::planet::shared::vector::{ui::line::ui::angle::ui::triangle::Triangle, Vector};

impl Display for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (a, b, c) = (self.abc.ab.a, self.abc.ab.b, self.abc.bc.b);
        // let t = PointDistribution::from(vec![a, b, c]);
        // let r = t.get_max_point();
        // let y = t.get_min_point();

        const MAX_WIDTH: i32 = 10;
        const MAX_HEIGHT: i32 = 10;
        const MAX_PADDING: i32 = 2;
        let max = Vector::from([
            [a, b, c]
                .into_iter()
                .max_by(|&a, &b| a[0].partial_cmp(&b[0]).unwrap())
                .unwrap()[0],
            [a, b, c]
                .into_iter()
                .max_by(|&a, &b| a[1].partial_cmp(&b[1]).unwrap())
                .unwrap()[1],
        ]);
        let min = Vector::from([
            [a, b, c]
                .into_iter()
                .min_by(|&a, &b| a[0].partial_cmp(&b[0]).unwrap())
                .unwrap()[0],
            [a, b, c]
                .into_iter()
                .min_by(|&a, &b| a[1].partial_cmp(&b[1]).unwrap())
                .unwrap()[1],
        ]);
        dbg!(max, min);
        let factor = Vector::from([
            (MAX_WIDTH - MAX_PADDING - 1) as f64,
            (MAX_HEIGHT - MAX_PADDING - 1) as f64,
        ]) / dbg!(max - min);
        dbg!(factor);
        let t = {
            let t: Vec<Vector> = dbg!([a, b, c]).into_iter().map(|v| v - min).collect();
            let t: Vec<Vector> = dbg!(t).into_iter().map(|v| v * factor).collect();
            let t: Vec<Vector<i32>> = t
                .into_iter()
                .map(|v| Vector::from([v[0] as i32, v[1] as i32]) + MAX_PADDING)
                .collect();
            t
        };
        dbg!(self.bac.get_normal());
        // let u: Vec<_> = self.iter().map(|bac|  {
        //     // Vector::from([bac.get_normal().cos(),
        //     // bac.get_normal().sin()]) * (2.0_f32.sqrt() as f64) + bac.ab.b
        //     bac.get_normal()
        // }).collect();
        // dbg!(u);
        let field = (0..MAX_HEIGHT + MAX_PADDING)
            .map(|h_i| {
                (0..MAX_WIDTH + MAX_PADDING)
                    .map(|w_i| {
                        let r: Option<usize> = t
                            .iter()
                            .map(|v| [v[0] - 1, v[1]])
                            .position(|v| v == [w_i, h_i]);
                        if let Some(i) = r {
                            return ["a", "b", "c"][i];
                        }

                        if t.contains(&Vector::from([w_i, h_i])) {
                            return "●";
                        }

                        if (h_i < MAX_PADDING || w_i < MAX_PADDING)
                            || (h_i >= MAX_HEIGHT || w_i >= MAX_WIDTH)
                        {
                            return "□";
                        }
                        " "
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", field)
    }
}