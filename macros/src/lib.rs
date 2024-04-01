use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn svg_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let name = &input.sig.ident;
    let block = &input.block;

    let output = quote! {
        #[test]
        fn #name() {
            use svg::Document;
            use std::{env, path::{Path, PathBuf}};
            use crate::planet::shared::traits::Svg;
            use svg::node::element::Polygon;

            let mut document = Document::new()
                .set("viewBox", (0, 0, 15, 15))
                .set("width", "1080")
                .set("height", "1080")
                .set("style", "transform: scaleY(-1); border: 10px black solid;");

            #block.to_svg(&mut document);
            let path = {
                let module_path_str = module_path!().to_string().replace("::", "\\");
                let path: PathBuf = Path::new(&module_path_str).components().skip(1).collect();
                let path = env::current_dir().unwrap().join("src").join(&path).join("svg_tests");
                std::fs::create_dir_all(&path).unwrap();
                path.join(format!("{}", stringify!(#name)) + ".svg")
            };
            svg::save(path.to_str().unwrap(), &document).unwrap();
        }
    };

    output.into()
}
