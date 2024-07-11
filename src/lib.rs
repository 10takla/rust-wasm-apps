#![allow(unused)]
#![feature(specialization)]

// mod game_of_life;
// pub mod planet;
// mod utils;
pub mod traits;

mod turbo {
    use crate::traits::of_to::{Of, To};
    use macros::{extended_structure, of_to, Default};
    use std::rc::Rc;

    #[derive(Default)]
    struct Tmp{
        #[default_field = 10]
        tmp: u32
    }


    // #[extended_structure(ab, bc, ca)]
    // struct Dota<T>(T);
    // #[extended_structure]
    // struct Dota<T, const N: usize>{
    //     pub a: [T; N],
    //     pub b: [T; N],
    //     pub c: [T; N]
    // }

    // #[derive(Clone, Debug, Copy)]
    // struct OfType;
    // #[derive(Clone, Debug, Copy)]
    // struct ToType;

    // #[of_to]
    // impl Of<OfType> for [Rc<ToType>; 2] {
    //     fn of(arg: OfType) -> Self {
    //         [Rc::new(ToType), Rc::new(ToType)]
    //     }
    // }

    // use std::collections::HashMap;

    // impl<const N: usize> Projection for [f64; N] {
    //     type Output<const P: usize> = [f64; P];
    //     fn projection<const P: usize>(&self, on: &[usize]) -> Self::Output<P> {
    //         Self::check_errors::<N, P>(on);

    //         self.into_iter()
    //             .enumerate()
    //             .filter(|(i, _)| !on.contains(i))
    //             .map(|(_, o)| *o)
    //             .collect::<Vec<f64>>()
    //             .try_into()
    //             .unwrap()
    //     }
    // }

    // #[test]
    // fn pro() {
    //     let t = [0.., 3..].projection::<2>(&[1, 3]);
    //     dbg!(t);
    // }
}
