use crate::planet::shared::traits::Svg;
use std::{
    env,
    path::{Path, PathBuf},
};
use svg::node::element::Polygon;
use svg::Document;

pub fn draw_svg<'a>(items: Vec<&'a (dyn Svg + 'static)>, name: &str, path: &str, extra_path: &str) {
    let mut document = Document::new()
        .set("viewBox", (0, 0, 12, 12))
        .set("width", "1080")
        .set("height", "1080");
    
    for item in items {
        item.to_svg(&mut document);
    }
    
    let path = {
        let module_path_str = path.to_string().replace("::", "\\");
        let path: PathBuf = Path::new(&module_path_str).components().skip(1).collect();
        let path = env::current_dir()
            .unwrap()
            .join("src")
            .join(&path)
            .join(extra_path)
            .join("svg_tests");
        std::fs::create_dir_all(&path).unwrap();
        path.join(format!("{name}.svg"))
    };
    svg::save(path.to_str().unwrap(), &document).unwrap();
}
