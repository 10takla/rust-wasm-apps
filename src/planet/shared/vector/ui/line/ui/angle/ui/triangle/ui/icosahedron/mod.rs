use std::{iter::once, ops::{Deref, DerefMut}, rc::Rc};

use crate::{
    planet::shared::{
        point::{DefaultMeasureValue, Point, DEFAULT_MEASURE},
        traits::{As, Normalize, Projection, Svg},
        vector::{
            ui::line::{ui::angle::ui::triangle::TriangleType, LineType},
            Number, Vector, VectorType,
        },
    },
    traits::of_to::{Of, To},
    utils::svg::draw_svg,
};
use macros::Deref;
use svg::node::Value;

#[derive(Deref, Debug, Clone, Default, PartialEq)]
pub struct Path<T: Clone = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE>(
    pub Vec<LineType<T, N>>,
);

impl<T: Number, const N: usize> Of<Vec<LineType<T, N>>> for Path<T, N> {
    fn of(lines: Vec<LineType<T, N>>) -> Self {
        Self(lines)
    }
}

impl<T: Number, const N: usize> Of<Vec<VectorType<T, N>>> for Path<T, N> {
    fn of(vecs: Vec<VectorType<T, N>>) -> Self {
        if vecs.len() < 2 {
            return Path::default();
        }
        Self(
            (0..vecs.len() - 1)
                .map(|i| LineType::of([&vecs[i], &vecs[i + 1]]))
                .collect(),
        )
    }
}

impl<T: Number, const N: usize> Of<Path<T, N>> for Vec<VectorType<T, N>> {
    fn of(path: Path<T, N>) -> Self {
        path.iter().map(|line| line.a.clone()).collect()
    }
}

impl<T: Number, const N: usize> Of<Path<T, N>> for Vec<Point<T, N>> {
    fn of(path: Path<T, N>) -> Self {
        path.to::<Vec<VectorType<T, N>>>().to()
    }
}

impl<T: Number, const N: usize> Of<&Path<T, N>> for Vec<Point<T, N>> {
    fn of(path: &Path<T, N>) -> Self {
        path.clone().to()
    }
}

// impl<T: Number, const N: usize, I: Of<Vec<VectorType<T, N>>>> Of<Path<T, N>> for I {
//     fn of(path: Path<T, N>) -> Self {
//         path.to::<Vec<VectorType<T, N>>>().to()
//     }
// }

trait Subdivide {
    type Output;
    fn subdivide(&self, resolution: usize) -> Self::Output;

    fn check_resolution(resolution: usize) -> bool {
        if resolution == 0 {
            return true;
        }
        false
    }
}

impl<T: Number, const N: usize> Subdivide for LineType<T, N> {
    type Output = Path<T, N>;
    fn subdivide(&self, resolution: usize) -> Self::Output {
        let vec = self.to::<VectorType<T, N>>();

        if (resolution == 0) {
            return Path(vec![self.clone()]);
        }

        once(self.a.clone())
            .chain((1..=resolution).map(|i| {
                let k = (i as f64 / (resolution + 1) as f64);
                Rc::new((vec.as_::<f64>() * k).as_::<T>() + *self.a)
            }))
            .chain(once(self.b.clone()))
            .collect::<Vec<_>>()
            .to()
    }
}

#[test]
fn subdivide_line() {
    let check = |resolution, points| {
        let line = LineType::of([[0., 0.], [12., 12.]]);
        assert_eq!(line.subdivide(resolution).to::<Vec<Point<_, 2>>>(), points)
    };
    check(1, vec![[0.0, 0.0], [6.0, 6.0]]);
    check(2, vec![[0.0, 0.0], [4.0, 4.0], [8.0, 8.0]]);
}

impl<T: Number, const N: usize> Subdivide for TriangleType<T, N> {
    type Output = Vec<Self>;
    fn subdivide(&self, resolution: usize) -> Self::Output {
        if Self::check_resolution(resolution) {
            return vec![self.clone()];
        }

        let vecs = self.to::<[VectorType<T, N>; 3]>();
        let lines = self.to::<[LineType<T, N>; 3]>().map(|line| {
            let [line_a, _]: [LineType<T, N>; 2] = line.subdivide(1).0.try_into().unwrap();
            line_a.b.clone()
        });

        vec![
            TriangleType::of([&vecs[1], &lines[1], &lines[0]]),
            TriangleType::of([&vecs[2], &lines[1], &lines[2]]),
            TriangleType::of([&vecs[0], &lines[0], &lines[2]]),
            TriangleType::of([&lines[1], &lines[2], &lines[0]]),
        ]
        .into_iter()
        .map(|tri| tri.subdivide(resolution - 1))
        .flatten()
        .collect()
    }
}

#[test]
fn subdivide_triangle() {
    let tri = TriangleType::of([[0., 0.], [12., 12.], [12., 0.]]);
    let tries = &tri.subdivide(2);
    draw_svg(vec![tries], "tmp", module_path!(), "tests");
}

#[derive(Deref, Clone)]
pub struct Icosahedron(Vec<TriangleType<f64, 3>>);

impl Icosahedron {
    pub fn new(resolution: usize, radius: f64) -> Self {
        let ico = Icosaedr::new();
        Self(
            ico.0
                .into_iter()
                .map(|tri| {
                    let mut tri = tri.subdivide(resolution);
                    // tri[0].to::<Vec<VectorType<f64, 3>>>();
                    tri[0].normalize();
                    tri
                })
                .flatten()
                .collect(),
        )
    }
}
impl<T: Number, const N: usize, I> Normalize<T, N> for I
where
    I: Of<Vec<VectorType<T, N>>> + Clone,
    Vec<VectorType<T, N>>: Of<I>,
{
    fn normalize(&mut self) -> &mut Self {
        let p = self.clone();

        let y = p.to::<Vec<VectorType<T, N>>>()
            .into_iter()
            .map(|mut vec| {
                let mut vec = *vec;
                vec.normalize();
                vec.clone()
            })
            .map(|vec| Rc::new(vec))
            .collect::<Vec<_>>();
        let s = y.to::<I>();

        *self = s;
        self
    }
}
// impl<T: Number, const N: usize> Normalize for TriangleType<T, N> {
//     fn normalize(&mut self) -> &mut Self {

//     }
// }

#[derive(Deref, Clone)]
pub struct Icosaedr([TriangleType<f64, 3>; 20]);

impl Icosaedr {
    pub fn new() -> Self {
        let t = (1. + (5. as f32).sqrt()) as f64 / 2.0;
        let vecs = vec![
            VectorType::of([-1., t, 0.]),
            VectorType::of([1., t, 0.]),
            VectorType::of([-1., -t, 0.]),
            VectorType::of([1., -t, 0.]),
            VectorType::of([0., -1., t]),
            VectorType::of([0., 1., t]),
            VectorType::of([0., -1., -t]),
            VectorType::of([0., 1., -t]),
            VectorType::of([t, 0., -1.]),
            VectorType::of([t, 0., 1.]),
            VectorType::of([-t, 0., -1.]),
            VectorType::of([-t, 0., 1.0]),
        ];

        Self(
            [
                [0, 11, 5],
                [0, 5, 1],
                [0, 1, 7],
                [0, 7, 10],
                [0, 10, 11],
                [1, 5, 9],
                [5, 11, 4],
                [11, 10, 2],
                [10, 7, 6],
                [7, 1, 8],
                [3, 9, 4],
                [3, 4, 2],
                [3, 2, 6],
                [3, 6, 8],
                [3, 8, 9],
                [4, 9, 5],
                [2, 4, 11],
                [6, 2, 10],
                [8, 6, 7],
                [9, 8, 1],
            ]
            .map(|[a, b, c]| TriangleType::of([&vecs[a], &vecs[b], &vecs[c]])),
        )
    }
}
