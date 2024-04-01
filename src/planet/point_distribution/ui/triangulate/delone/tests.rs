use super::Delone;
use crate::{
    planet::{
        point_distribution::{
            ui::{convex_hull::ConvexHull, triangulate::Triangles},
            PointDistribution,
        },
        shared::{
            point::Point,
            traits::Svg,
            vector::{ui::line::Line, Number, Vector},
        },
    },
    traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    },
};
use macros::svg_test;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
};
use svg::Document;

#[svg_test]
fn get_delone() {
    let pd: PointDistribution = vec![
        [3.0, 2.0],
        [2.0, 3.0],
        [2.0, 1.0],
        [5.0, 2.0],
        [4.0, 4.0],
        [6.0, 4.0],
        [7.0, 1.0],
        [7.5, 7.0],
        [8.0, 4.0],
        // [10.0, 4.0],
    ]
    .to();

    Delone::triangulate(&pd)
}

#[svg_test]
fn get_delone_by_random() {
    PointDistribution::set_random_points(4, [10.0, 10.0]).triangulate()
}

#[test]
fn get_start_point() {
    let check = |start: Vec<Point>, ves: Vec<[usize; 2]>| {
        let pd = start.to::<PointDistribution>();
        let rects = Delone::get_start_point(&pd.sort_points_by_min());

        ves.into_iter().enumerate().for_each(|(i, ves)| {
            assert_eq!(*rects[i].get_common_line(), Line::of(ves.map(|i| &pd[i])));
        });
    };

    check(
        vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0], [5.0, 2.0], [4.0, 4.0]],
        vec![[0, 1]],
    );

    check(
        vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0], [5.0, 2.0]],
        vec![[2, 0], [1, 0]],
    );

    check(
        vec![[1.0, 2.0], [2.0, 3.0], [2.0, 1.0], [5.0, 2.0]],
        vec![[2, 1]],
    );
}

#[test]
fn get_scope_vecs() {
    let check_lines = |points: Vec<Point>, rel_vec, finish_inds: Vec<usize>| {
        let pd = points.clone().to::<PointDistribution>();
        let lines = <PointDistribution as Delone>::get_scope_lines(
            pd.clone().convex_hull(),
            &Rc::new(Vector::of(rel_vec)),
        );

        assert_eq!(
            lines.clone(),
            finish_inds
                .into_iter()
                .map(|i| pd[i].clone())
                .collect::<Vec<Rc<Vector>>>()
                .to::<Vec<Rc<Line>>>()
        );
    };
    check_lines(
        vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
        [5.0, 2.0],
        vec![2, 0, 1],
    );
    check_lines(
        vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
        [4.0, 4.0],
        vec![0, 1],
    );
    check_lines(
        vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
        [5.0, 2.0],
        vec![2, 0, 1],
    );
    check_lines(
        vec![
            [2.5, 0.98],
            [2.83, 3.0],
            [3.0, 2.0],
            [2.25, 0.56],
            [2.63, 3.52],
            [2.36, 4.02],
            [2.03, 2.46],
            [1.92, 1.61],
            [2.0, 3.24],
            [2.4, 2.98],
            [2.33, 1.66],
        ],
        [4.95, 2.27],
        vec![3, 0, 2, 1, 4, 5],
    );
}

#[test]
#[ignore]
fn get_scope_vecs_demo() {
    let mut document = Document::new().set("viewBox", (0, 0, 200, 200));
    let mut check_lines = |points: Vec<Point>, rel_vec, finish_inds: Vec<usize>| {
        let pd: PointDistribution = points.clone().to();
        let lines = PointDistribution::get_scope_lines(
            pd.clone().convex_hull(),
            &Rc::new(Vector::of(rel_vec)),
        );
        assert_eq!(
            lines.clone(),
            finish_inds
                .into_iter()
                .map(|i| pd[i].clone())
                .collect::<Vec<Rc<Vector>>>()
                .to::<Vec<Rc<Line>>>()
        );

        lines
            .into_iter()
            .map(|line| (*line).clone() * 40.0)
            .collect::<Vec<Line>>()
            .to_svg(&mut document)
    };
    // check_lines(
    //     vec![[3.0, 2.0], [2.0, 3.0], [2.0, 1.0]],
    //     [5.0, 2.0],
    //     vec![2, 0, 1],
    // );
    check_lines(
        vec![[3.0, 2.0], [2.0, 3.0], [2.5, 1.0], [2.0, 0.5]],
        [5.0, 2.0],
        vec![3, 2, 0, 1],
    );
    // check_lines(
    //     vec![
    //         [2.5, 0.98],
    //         [2.83, 3.0],
    //         [3.0, 2.0],
    //         [2.25, 0.56],
    //         [2.63, 3.52],
    //         [2.36, 4.02],
    //         [2.03, 2.46],
    //         [1.92, 1.61],
    //         [2.0, 3.24],
    //         [2.4, 2.98],
    //         [2.33, 1.66],
    //     ],
    //     [4.95, 2.27],
    //     vec![3, 0, 2, 1, 4, 5],
    // );

    let test_name = "get_scope_vecs";
    let path = {
        let module_path_str = module_path!().to_string().replace("::", "\\");
        let path: PathBuf = Path::new(&module_path_str).components().skip(1).collect();
        let path = env::current_dir().unwrap().join("src").join(&path);
        std::fs::create_dir_all(&path).unwrap();
        path.join(test_name.to_owned() + ".svg")
    };
    svg::save(path.to_str().unwrap(), &document).unwrap();
    Command::new("cmd")
        .args(&["/C", "start", "firefox"])
        .arg(path.to_str().unwrap())
        .spawn()
        .expect("Не удалось запустить браузер по умолчанию.");
}
