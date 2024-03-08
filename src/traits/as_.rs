use paste::paste;

macro_rules! every_type_method {
    ($($t:ty),+) => {
        $(
            paste! {
                fn [<to_ $t>](&self) -> $t {
                    *self as $t
                }
            }
        )+
    };
}

macro_rules! impl_ToPrim_for_every_types {
    ($($t:ty),+) => {
        pub trait ToPrim {
            $(
                paste! {
                    fn [<to_ $t>](&self) -> $t;
                }
            )+
        }
        $(
            impl ToPrim for $t {
                every_type_method!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
            }
        )+
        $(
            impl FromPrim for $t {
                paste! {
                    fn from<F: ToPrim>(value: F) -> $t {
                        value.[<to_ $t>]()
                    }
                }
            }
        )+
    }
}

pub trait FromPrim: ToPrim {
    fn from<F: ToPrim>(value: F) -> Self;
}

impl_ToPrim_for_every_types!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

pub trait As: FromPrim {
    fn as_<T: FromPrim>(self) -> T;
}

impl<F: FromPrim> As for F {
    fn as_<I: FromPrim>(self) -> I {
        I::from::<F>(self)
    }
}
