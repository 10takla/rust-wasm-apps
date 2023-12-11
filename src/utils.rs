
#[macro_export]
macro_rules! print_expr {
  ($eval:tt) => {
      println!("{}", stringify!($eval));
  };
}