use crate::{planet::{point_distribution::ui::triangulate::Triangulate, shared::traits::{As, Normalize}}, traits::of_to::Of};

use std::{any::type_name, mem, time::Instant};

use crate::{
    planet::{
        point_distribution::PointDistribution,
        shared::vector::ui::line::{ui::angle::Angle, Line},
    },
    traits::as_prim::AsPrim,
};

use super::{ui::line::ui::angle::ui::triangle::Triangle, Number, Vector};

#[test]
fn angle() {
    assert_eq!(Vector([1, 1]).angle(), 45);
    assert_eq!(Vector([-1, 1]).angle(), 135);
    assert_eq!(Vector([1, -1]).angle(), -45);
    assert_eq!(Vector([-1, -1]).angle(), -135);
}

#[test]
fn vector_from() {
    let vector: Vector<i32> = Vector::of([1; 2]);
    {
        let v_f64: Vector<f64> = vector.as_();
        assert_eq!(*v_f64, [1.0; 2]);
    }
}

#[test]
fn normalize() {
    assert_eq!(Vector::of([0.4, 0.]).normalize().0, Vector::of([1., 0.]).0);
    assert_eq!(Vector::of([0., 0.2]).normalize().0, Vector::of([0., 1.]).0);
}

#[test]
#[ignore]
fn perfomance_vector_system() {
    fn check_size<T: Number>() {
        let point = [1.0.as_::<T>(); 3];
        let triangle = Triangle::of([point; 3]);
        let angle = Angle::of([point; 3]);
        let line = Line::of([point; 2]);
        let vector = Vector::of(point);
        println!("{}", type_name::<T>());
        macro_rules! to_mem {
            ($($el:ident),+) => {
                $(
                    println!("{}: {}", stringify!($el), mem::size_of_val(&$el));
                )+
            };
        }
        to_mem!(triangle, angle, line, vector);
    }
    macro_rules! to_size {
        ($($t:ty),+) => {
            $(
                check_size::<$t>();
                println!("");
            )+
        };
    }
    to_size!(f64, i32, i128);

    macro_rules! to_pref {
        ($($discr:expr => $test:block),+ $(,)?) => {
            $(
                let start = Instant::now();
                $test
                println!("{} {:?}", $discr, Instant::now() - start);
            )+
        };
    }
    to_pref!(
        "create items" => {
            let vector = Vector::of([1; 3]);
            let t = Triangle::of([vector; 3]);
            let t = Angle::of([vector; 3]);
            let t = Line::of([vector; 2]);
        },
        "create circle from traingle" => {
            Triangle::of([[2.0, 3.0], [4.0, 8.0], [1.0, 1.0]]).get_circle();
        },
        // "trinagulate" => {
        //     PointDistribution::set_random_points(2000, [[0.0,0.0],[1.0; 2]]).triangulate();
        // }
    );
}
