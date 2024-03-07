use crate::planet::shared::vector::{
    ui::line::ui::angle::{ui::triangle::Triangle, Angle},
    Number, Vector,
};
use core::fmt;
use std::fmt::{Display, Formatter};

impl<T: Number + PartialOrd + Into<i32> + From<i32> + Into<f64> + From<f64>> Display
    for Triangle<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (a, b, c) = (self.cab.ba.a.as_::<i32>(), self.abc.ba.a.as_::<i32>(), self.bca.ba.a.as_::<i32>());

        const MAX_WIDTH: i32 = 20;
        const MAX_HEIGHT: i32 = 20;
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

        let vertices: [Vector<i32>; 3] = {
            let factor: Vector<i32> = Vector::from([
                (MAX_WIDTH - MAX_PADDING / 2),
                (MAX_HEIGHT - MAX_PADDING / 2),
            ]) / (max - min);

            [a, b, c]
                .into_iter()
                .map(|v| (v - min) * factor)
                .map(|v| v + MAX_PADDING)
                .collect::<Vec<_>>()
                .try_into()
                .expect("Expected a Vec of length 3")
        };
        // dbg!(self.bac, self.bac.get_normal());
        let u: Vec<Vector<i32>> = self
            .iter()
            .map(|cab| {
                let r = cab.get_normal().angle();
                let r = (r + 180.into()) % 360.into();
                let v =  Angle::angle_to_vector(r).as_()+ cab.ba.a;
                v.as_() + MAX_PADDING
            })
            .collect();

        let (height, width) = (MAX_HEIGHT + MAX_PADDING * 2, MAX_WIDTH + MAX_PADDING * 2);
        let field = (0..height)
            .map(|h_i| {
                (0..width)
                    .map(|w_i| {
                        let h_i = height - h_i - 1;
                        let r: Option<usize> = u
                            .iter()
                            .map(|v| [v[0] - 1, v[1]])
                            .position(|v| v == [w_i, h_i]);
                        if let Some(i) = r {
                            return ["a", "b", "c"][i];
                        }

                        if vertices.contains(&Vector::from([w_i, h_i])) {
                            return "●";
                        }

                        if (h_i < MAX_PADDING || w_i < MAX_PADDING)
                            || (h_i >= MAX_HEIGHT + MAX_PADDING || w_i >= MAX_WIDTH + MAX_PADDING)
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
