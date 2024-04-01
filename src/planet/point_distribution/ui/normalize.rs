use std::ops::Deref;

use crate::planet::point_distribution::PointDistribution;

impl PointDistribution {
    pub fn normalize_points(&self) -> Vec<f64> {
        let r = self
            .iter()
            .enumerate()
            .map(|(i, target)| {
                let s: f64 = self
                    .iter()
                    .skip(i)
                    .map(|other| (**other - **target).angle())
                    .sum();
                s / self.len() as f64
            })
            .collect();
        r
    }
}

#[test]
fn normalize_points() {
    let r: PointDistribution = vec![[1.0, 1.0]].to();
    let r = r.normalize_points();
}


#[test]
#[ignore]
fn tmp() {
    async fn as_ex(points: Vec<Vector>, thread_count: usize) -> Vector {
        let count = points.len();
        let start = Instant::now();
        let ave_vec = |points| {
            points
                .iter()
                .sum::<Vector>()
        };
        let mut ave_vecs = Vec::new();
        for i in 0..thread_count {
            let points = points[i * count / thread_count..(i + 1) * count / thread_count].to_vec();
            let thread = async {
                // println!("{i}");
                ave_vec(points)
            };
            ave_vecs.push(thread);
        }
        let ave_vecs = join_all(ave_vecs).await;
        let center = ave_vec(ave_vecs) / count as f64;

        let end = Instant::now();

        // println!(
        //     "{thread_count} потоков {} точек за {:?}, результат: {center:?} ",
        //     points.len(),
        //     end - start,
        // );
        center
    }

    let ex = |count, thread_count, c| {
        let common_start = Instant::now();
        let points: Vec<Vector> = {
            let sizes = [1.0, 1.0];
            (0..count)
                .map(|_| {
                    let start = 0.0;
                    let mut measures = [start; 2];
                    for i in 0..2 {
                        measures[i] = if sizes[i] != start {
                            rand::thread_rng().gen_range(start..sizes[i])
                        } else {
                            start
                        }
                    }
                    Vector(measures)
                })
                .collect()
        };

        let start = Instant::now();
        let ave_vec = |points| {
            points
                .iter()
                .sum::<Vector>()
        };
        let mut ave_vecs = Vec::new();
        for i in 0..thread_count {
            let points = points[i * count / thread_count..(i + 1) * count / thread_count].to_vec();
            let thread = thread::spawn(move || {
                block_on(as_ex(points, c))
            });
            ave_vecs.push(thread);
        }
        let ave_vecs = ave_vecs.into_iter().map(|th| th.join().unwrap()).collect();
        let center = ave_vec(ave_vecs) / count as f64;

        let end = Instant::now();

        println!(
            "{thread_count} потоков {count} точек за {:?}, общее: {:?}, результат: {center:?} ",
            end - start,
            end - common_start
        );
    };
    ex(1_000_000, 1, 10);
    ex(1_000_000, 10, 10);
    ex(1_000_000, 40, 10);
    println!("");
    ex(1_000_000, 100, 10);
    ex(1_000_000, 100, 40);

    println!("");
    ex(10_000_000, 100, 10);
    ex(10_000_000, 100, 40);
}


