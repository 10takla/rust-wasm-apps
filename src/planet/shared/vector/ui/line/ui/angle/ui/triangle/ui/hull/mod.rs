use crate::{
    planet::shared::{
        point::{DefaultMeasureValue, Point, DEFAULT_MEASURE},
        vector::{
            ui::line::{Line, LineType},
            Number, Vector, VectorType,
        },
    },
    traits::of_to::{Of, To},
};
use macros::{of_to, Deref};
use std::fmt::Debug;
use std::rc::Rc;

use super::icosahedron::Path;

#[derive(Clone, Default, Deref, PartialEq)]
pub struct Hull<T: Clone = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE>(pub Path<T, N>);

impl<T: Number, const N: usize, F> Of<F> for Hull<T, N>
where
    Path<T, N>: Of<F>,
{
    fn of(arg: F) -> Self {
        let mut path = arg.to::<Path<T, N>>();
        if path.len() > 1 {
            let last = LineType::of([&path[path.len() - 1].b, &path[0].a]);
            path.push(last);
        }
        Hull(path)
    }
}

impl<T: Number, const N: usize, I: Of<Path<T, N>>> Of<Hull<T, N>> for I {
    fn of(hull: Hull<T, N>) -> Self {
        hull.0.to()
    }
}
impl<T: Number, const N: usize, I: Of<Path<T, N>>> Of<&Hull<T, N>> for I {
    fn of(hull: &Hull<T, N>) -> Self {
        hull.0.clone().to()
    }
}

// impl<T: Number, const N: usize> Of<Vec<VectorType<T, N>>> for Hull<T, N> {
//     fn of(vecs: Vec<VectorType<T, N>>) -> Self {
//         let mut y = vecs;
//         y.push(y[0].clone());
//         Self(y.to())
//     }
// }

impl<T: Number, const N: usize> Of<&Vec<VectorType<T, N>>> for Hull<T, N> {
    fn of(vecs: &Vec<VectorType<T, N>>) -> Self {
        vecs.clone().to()
    }
}

impl<T: Number, const N: usize> Of<Vec<&VectorType<T, N>>> for Hull<T, N> {
    fn of(vecs: Vec<&VectorType<T, N>>) -> Self {
        vecs.into_iter()
            .map(|vector| vector.clone())
            .collect::<Vec<VectorType<T, N>>>()
            .to()
    }
}

// impl<T: Number, const N: usize> Of<&Hull<T, N>> for Vec<VectorType<T, N>> {
//     fn of(vecs: &Hull<T, N>) -> Self {
//         vecs.0
//             .iter()
//             .map(|line| line.clone())
//             .collect::<Vec<LineType<T, N>>>()
//             .to()
//     }
// }

impl<T: Number, const N: usize> Debug for Hull<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.to::<Vec<VectorType<T, N>>>().into_iter())
            .finish()
    }
}

#[test]
fn hull_of_vecs() {
    let check = |points: Vec<Point<i32, 1>>, len| {
        let t = dbg!(Hull::of(&points.to::<Vec<VectorType<i32, 1>>>()).0);
        assert_eq!(t.len(), len);
    };

    check(vec![[1], [2], [3], [4], [5], [6], [7], [8], [9]], 9);
    check(vec![[1], [2], [3]], 3);
    check(vec![[1], [2]], 1);
    check(vec![[1]], 0);
    check(vec![], 0);
}
