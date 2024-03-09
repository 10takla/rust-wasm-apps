use crate::planet::{
    point_distribution::PointDistribution,
    shared::{
        point::Point,
        vector::{
            ui::line::{ui::angle::Angle, Line}, Number, Vector
        },
    },
};
use std::{f32::consts::PI, fmt::Display, rc::Rc};

impl<T: Number> Display for Line<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let (width, height, padding) = (10, 10, 1);

        // let (min, max) = PointDistribution::from(vec![self.a, self.b]).get_box_boundary();
        // let height = if max[1] > T::from(height) {
        //     height
        // } else {
        //     let v = max.as_()[1];
        //     if v == 0 {
        //         1
        //     } else {
        //         v
        //     }
        // };

        // let size = Vector::from([width, height]);

        // let line: Line<i32> = {
        //     let line: Line<f64> = self.as_();
        //     let (max, min) = (max.as_(), min.as_());
        //     let length = Vector::from({
        //         let point: Point<f64> = (max - min)
        //             .into_iter()
        //             .map(|v| if v == 0.0 { 1.0 } else { v })
        //             .collect::<Vec<f64>>()
        //             .try_into()
        //             .unwrap();
        //         point
        //     });
        //     let factor: Vector<f64> = (size - 1).as_() / length;

        //     Line::from({
        //         let vecs: [Vector<i32>; 2] = line
        //             .into_iter()
        //             .map(|v| ((*v - min) * factor + padding as f64))
        //             .map(|v| v.as_())
        //             .collect::<Vec<Vector<i32>>>()
        //             .try_into()
        //             .unwrap();
        //         vecs
        //     })
        // };

        // let prefs = {
        //     let get_normal = |line: Line<i32>| {
        //         let t = *line.a;
        //         (Angle::<f64>::angle_to_vector(Vector::from(line).as_::<f64>().atan() + PI as f64) * 2.0).as_() + t
        //     };
        //     [get_normal(line), get_normal(line.reverse())]
        // };

        // let [a, b] = [*line.a, *line.b];

        // let slashes: Vec<Vector<i32>> = {
        //     let interp_count = (a - b)[1].abs() + 6;
        //     (0..dbg!(interp_count))
        //         .into_iter()
        //         .map(|i| ((a - b) * i / interp_count) + b)
        //         .collect()
        // };
        // let slash_type = if (b - a).angle() > 0 { r"/" } else { r"\" };

        // let size = size + padding * 2;

        // let field = (0..size[1])
        //     .rev()
        //     .map(|h_i| {
        //         (0..size[0])
        //             .map(|w_i| {
        //                 type P = [Vector<i32>; 2];
        //                 let v = Vector([w_i, h_i]);
        //                 if P::from(line).contains(&v) {
        //                     return "●";
        //                 }
        //                 if let Some(i) = prefs.into_iter().position(|p| p.0 == [w_i, h_i]) {
        //                     return ["a", "b"][i];
        //                 }
        //                 if (&slashes).contains(&v) {
        //                     return slash_type;
        //                 }
        //                 if w_i < padding
        //                     || h_i < padding
        //                     || w_i >= width + padding
        //                     || h_i >= height + padding
        //                 {
        //                     return " ";
        //                 }
        //                 " "
        //             })
        //             .collect::<Vec<&str>>()
        //             .join(" ")
        //     })
        //     .collect::<Vec<String>>()
        //     .join("\n");

        write!(f, "")
    }
}
