use crate::planet::point_distribution::PointDistribution;

impl PointDistribution {
    pub fn normalize_points(&self) -> Vec<f64> {
        let r = self.iter().enumerate().map(|(i, &target)| {
            let s: f64 = self.iter().skip(i).map(|&other| {
                (other - target).angle()
            }).sum();
            s / self.len() as f64
        }).collect();
        r
    }
}

#[test]
fn normalize_points() {
    let r: PointDistribution = vec![[1.0, 1.0]].into();
    let r = r.normalize_points();
    // dbg!(r);
}