use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[macro_export]
macro_rules! print_expr {
  ($eval:tt) => {
      println!("{}", stringify!($eval));
  };
}

#[macro_export]
macro_rules! derive_deref {
  ($struct_name:ident, $field_name:tt, $target_type:ty, $type:tt) => {
        impl<$type> std::ops::Deref for $struct_name<$type> {
            type Target = $target_type;

            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl<$type> std::ops::DerefMut for $struct_name<$type> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }
    };
    ($struct_name:ident, $field_name:tt, $target_type:ty) => {
        impl std::ops::Deref for $struct_name {
            type Target = $target_type;

            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl std::ops::DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }
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
