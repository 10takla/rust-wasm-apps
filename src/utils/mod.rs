pub mod svg;

#[macro_export]
macro_rules! print_expr {
    ($eval:tt) => {
        println!("{}", stringify!($eval));
    };
}

#[macro_export]
macro_rules! struct_by_keys {
    ($name:ident, $t:ty, $($key:ident),+) => {
        pub struct $name {
            $(
                $key: $t,
            )+
        }
    };
}
