use super::Triangles;
use crate::{
    planet::{
        point_distribution::{ui::convex_hull::ConvexHull, PointDistribution},
        shared::vector::{
            ui::line::ui::angle::{
                ui::triangle::{ui::rectangle::Rectangle, Triangle},
                Angle,
            },
            Vector,
        },
    },
    traits::{
        as_::As,
        of_to::{Of, To},
    },
};
use std::rc::Rc;

pub trait Delone {
    fn triangulate(&self) -> Triangles;
}

impl PointDistribution {
    fn get_next_point(&self, stack_rects: &mut Vec<Rectangle>) {
        let (i, nearest_vec) = ch
            .clone()
            .into_iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let [a, b] = [a, b]
                    .map(|v| (*next_vec - **v).radius().abs())
                    .try_into()
                    .unwrap();
                a.partial_cmp(&b).unwrap()
            })
            .unwrap();
        // Нахождение точек в поле видимости
        let r = ch[(i + 1) % ch.len()].clone();
        let [right, left] = if Angle::of([next_vec.clone(), nearest_vec.clone(), r.clone()])
            .get_polar_angle()
            > 0.0.as_()
        {
            [1, -1]
        } else {
            [-1, 1]
        };
        let mut scope = vec![];
        // левые точки поля видимости
        let mut left_scope = vec![];
        loop {
            let l = ch[(i as isize + left) as usize % ch.len()].clone();
            if Angle::of([next_vec.clone(), nearest_vec.clone(), l.clone()]).get_polar_angle() < 0.0
            {
                left_scope.push(l.clone());
            } else {
                break;
            }
            break;
        }
        // переворот левых точек, для порядка от левой к правой
        left_scope.into_iter().for_each(|v| scope.push(v));
        scope.push(nearest_vec.clone());
        // правые точки поля видимости
        loop {
            let r = ch[(i + right as usize) % ch.len()].clone();
            if Angle::of([next_vec.clone(), nearest_vec.clone(), r.clone()]).get_polar_angle() > 0.0
            {
                scope.push(r.clone());
            } else {
                break;
            }
            break;
        }
        // проведение треугольников к полю зрения
        if scope.len() == 2 {
            let tri = Triangle::of([scope[0].clone(), scope[1].clone(), next_vec.clone()]);
            let mut rect = Rectangle::of([start_tri.clone(), tri.clone()]);
            let v1 = start_tri.get_circle();
            if (*next_vec - *v1.center).radius() < v1.radius() {
                rect = rect.reverse_tries();
            }
            stack_rects.push(rect);
        } else if scope.len() >= 3 {
            let tri = Triangle::of([scope[0].clone(), scope[1].clone(), next_vec.clone()]);
            let rect = Rectangle::of([start_tri.clone(), tri.clone()]);
            stack_rects.push(rect);
            let tri = Triangle::of([scope[1].clone(), scope[2].clone(), next_vec.clone()]);
            let rect = Rectangle::of([start_tri.clone(), tri.clone()]);
            stack_rects.push(rect);
        }
    }
}

impl Delone for PointDistribution {
    fn triangulate(&self) -> Triangles {
        let points = self.sort_points_by_min();
        let start_tri = Triangle::of(points[0..=2].to_vec());
        let start_vecs = start_tri.clone().to::<[Rc<Vector>; 3]>();

        let ch = PointDistribution::of(start_vecs.clone().to_vec()).convex_hull();

        let next_vec = points[3].clone();
        
        
        let mut stack_rects: Vec<Rectangle> = vec![];
        self.get_next_point(&mut stack_rects);
        dbg!(stack_rects);
        // let l = ch[(i + left as usize) % ch.len()].clone();
        vec![]
    }
}
