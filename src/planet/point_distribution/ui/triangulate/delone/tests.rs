use super::Delone;
use crate::planet::shared::traits::SvgStyle;
use crate::planet::shared::vector::svg::VectorStyle;
use crate::{
    planet::{
        point_distribution::{
            ui::{convex_hull::ConvexHull, triangulate::Triangles},
            PointDistribution,
        },
        shared::{
            point::Point,
            traits::Svg,
            vector::{
                ui::line::{ui::angle::ui::triangle::TriangleType, Line},
                Number, Vector, VectorType,
            },
        },
    },
    traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    },
};
use macros::svg_test;
use std::{
    env,
    ops::Sub,
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
};
use svg::Document;

#[svg_test]
// #[ignore]
fn get_delone() {
    // PointDistribution::of(vec![
    //     [1, 3],
    //     [2, 7],
    //     [3, 1],
    //     [3, 8],
    //     [6, 6],
    //     [7, 2],
    //     [7, 9],
    //     [8, 6],
    // ])
    // .triangulate()
    let mut pd = PointDistribution::set_random_points(15, [[1.; 2], [10.; 2]]);
    let ts = pd.triangulate();
    let pd = pd - PointDistribution(ts.clone().to::<Vec<VectorType>>());
    let t: Vec<Box<dyn Svg>> = vec![
        Box::new(SvgStyle(pd.0, VectorStyle::default().set_color("yellow"))),
        Box::new(ts),
    ];
    t
}
impl<'a, T, const N: usize> FromIterator<&'a VectorType<T, N>> for PointDistribution<T, N> {
    fn from_iter<I: IntoIterator<Item = &'a VectorType<T, N>>>(iter: I) -> Self {
        PointDistribution(iter.into_iter().cloned().collect::<Vec<VectorType<T, N>>>())
    }
}

impl<T: PartialEq, const N: usize> Sub for PointDistribution<T, N> {
    type Output = PointDistribution<T, N>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.iter().filter(|&k| !rhs.contains(k)).collect()
    }
}

// #[test]
// fn get_scope_vecs() {
//     let check_lines = |points: Vec<Point>, rel_vec, finish_inds: Vec<usize>| {
//         let mut pd = points.to::<PointDistribution>();

//         assert_eq!(
//             finish_inds
//                 .into_iter()
//                 .map(|i| pd[i].clone())
//                 .collect::<Vec<Rc<Vector>>>()
//                 .to::<Vec<Rc<Line>>>(),
//             PointDistribution::get_scope_lines(pd.convex_hull().0, &Rc::new(Vector::of(rel_vec)))
//         );
//     };

//     check_lines(
//         vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
//         [5.0, 2.0],
//         vec![2, 0, 1],
//     );
//     check_lines(
//         vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
//         [4.0, 4.0],
//         vec![0, 1],
//     );
//     check_lines(
//         vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
//         [5.0, 2.0],
//         vec![2, 0, 1],
//     );

//     check_lines(
//         vec![[2.0, 1.0], [2.0, 3.0], [3.0, 2.0]],
//         [4.0, 1.0],
//         vec![0, 2],
//     );

//     check_lines(
//         vec![[2.0, 1.0], [2.0, 3.0], [3.0, 2.0], [4.0, 1.0]],
//         [4.0, 4.0],
//         vec![3, 2, 1],
//     );

//     check_lines(
//         vec![
//             [2.5, 0.98],
//             [2.83, 3.0],
//             [3.0, 2.0],
//             [2.25, 0.56],
//             [2.63, 3.52],
//             [2.36, 4.02],
//             [2.03, 2.46],
//             [1.92, 1.61],
//             [2.0, 3.24],
//             [2.4, 2.98],
//             [2.33, 1.66],
//         ],
//         [4.95, 2.27],
//         vec![3, 0, 2, 1, 4, 5],
//     );
// }
