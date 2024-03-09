use crate::planet::{point_distribution::PointDistribution, shared::vector::{
    ui::line::ui::angle::{ui::triangle::Triangle, Angle},
    Number, Vector,
}};
use core::fmt;
use std::{fmt::{Display, Formatter}, ops::Deref};
use crate::traits::as_::As;

impl<T: Number> Display for Triangle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (a, b, c) = (
            self.cab.ba.a.as_::<i32>(),
            self.abc.ba.a.as_::<i32>(),
            self.bca.ba.a.as_::<i32>(),
        );

        const MAX_WIDTH: i32 = 20;
        const MAX_HEIGHT: i32 = 20;
        const MAX_PADDING: i32 = 2;
        let (max, min) = PointDistribution::from(vec![a, b, c]).get_box_boundary();

        let vertices: [Vector<i32, 2>; 3] = {
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

        let prefs: Vec<Vector<i32>> = self
            .iter()
            .map(|cab| {
                let opposite_angle = (cab.get_normal().angle() + 180.as_()) % 360.as_();
                let v = Angle::<T>::angle_to_vector(opposite_angle).as_() + *cab.ba.a.clone();
                v.as_() + MAX_PADDING
            })
            .collect();

        let (height, width) = (MAX_HEIGHT + MAX_PADDING * 2, MAX_WIDTH + MAX_PADDING * 2);
        let field = (0..height)
            .map(|h_i| {
                (0..width)
                    .map(|w_i| {
                        let h_i = height - h_i - 1;
                        let r: Option<usize> = prefs
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
