use std::rc::Rc;

use crate::{planet::shared::{traits::{Has, Nearest}, vector::{ui::line::Line, Vector}}, traits::of_to::To};

impl Nearest<Rc<Vector>> for Vec<Rc<Line>> {
    fn nearest(&self, vector: &Rc<Vector>) -> Rc<Vector> {
        let vecs: Vec<Rc<Vector>> = self
            .into_iter()
            .map(|line| (*line).clone().to::<[Rc<Vector>; 2]>())
            .flatten()
            .collect();
        vecs.nearest(vector)
    }
}

impl Nearest<Rc<Vector>, Vec<Rc<Line>>> for Vec<Rc<Line>> {
    fn nearest(&self, vector: &Rc<Vector>) -> Vec<Rc<Line>> {
        let nearest_vector = self.nearest(vector);
        self.clone()
            .into_iter()
            .filter(|line| {
                (*line)
                    .clone()
                    .to::<[Rc<Vector>; 2]>()
                    .into_iter()
                    .any(|vector| Rc::ptr_eq(&vector, &nearest_vector))
            })
            .collect::<Vec<Rc<Line>>>()
    }
}

impl Nearest<Rc<Vector>, Rc<Line>> for Vec<Rc<Line>> {
    fn nearest(&self, vector: &Rc<Vector>) -> Rc<Line> {
        let nearest_vector: Rc<Vector> = self.nearest(vector);

        (*self)
            .clone()
            .into_iter()
            .filter(|line| line.has(&nearest_vector))
            .min_by(|ab, bc| {
                let [a, b] = [ab, bc].map(|line| {
                    line
                        .iter()
                        .find(|&vec| *vec != nearest_vector)
                        .unwrap()
                });
                a.partial_cmp(&b).unwrap()
            })
            .unwrap()
    }
}