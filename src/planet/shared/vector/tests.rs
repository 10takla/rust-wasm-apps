use std::{any::type_name, mem, rc::Rc, time::Instant};

use crate::{
    planet::{
        point_distribution::PointDistribution,
        shared::vector::ui::line::{ui::angle::Angle, Line},
    },
    traits::as_::As,
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
    let vector: Vector<i32> = Vector::from([1; 2]);
    {
        let v_f64: Vector<f64> = vector.as_();
        assert_eq!(*v_f64, [1.0; 2]);
    }
}

#[test]
#[ignore]
fn perfomance_vector_system() {
    fn check_size<T: Number>() {
        let point = [1.0.as_::<T>(); 3];
        let triangle = Triangle::from([point; 3]);
        let angle = Angle::from([point; 3]);
        let line = Line::from([point; 2]);
        let vector = Vector::from(point);
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
        ($($discr:expr => $test:block),+) => {
            $(
                let start = Instant::now();
                $test
                println!("{} {:?}", $discr, Instant::now() - start);
            )+
        };
    }
    to_pref!(
        "create items" => {
            let vector = Vector::from([1; 3]);
            let t = Triangle::from([vector; 3]);
            let t = Angle::from([vector; 3]);
            let t = Line::from([vector; 2]);
        },
        "create circle from traingle" => {
            Triangle::from([[2.0, 3.0], [4.0, 8.0], [1.0, 1.0]]).get_circle();
        },
        "trinagulate" => {
            PointDistribution::set_random_points(2000, [1.0; 2]).triangulate();
        }
    );
}
