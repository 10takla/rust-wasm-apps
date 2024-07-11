use crate::planet::shared::vector::ui::line::LineType;
use crate::planet::shared::vector::VectorType;
use crate::{
    planet::shared::{
        traits::{Has, Nearest},
        vector::{ui::line::Line, Number, Vector},
    },
    traits::of_to::To,
};
use std::rc::Rc;

impl<T: Number, const N: usize> Nearest<VectorType<T, N>> for Vec<LineType<T, N>> {
    fn nearest(&self, vector: &VectorType<T, N>) -> VectorType<T, N> {
        self.into_iter()
            .map(|line| (*line).clone().to::<[VectorType<T, N>; 2]>())
            .flatten()
            .collect::<Vec<VectorType<T, N>>>()
            .nearest(vector)
    }
}

impl<T: Number, const N: usize> Nearest<VectorType<T, N>, Vec<LineType<T, N>>>
    for Vec<LineType<T, N>>
{
    fn nearest(&self, vector: &VectorType<T, N>) -> Vec<LineType<T, N>> {
        let nearest_vector = self.nearest(vector);
        self.clone()
            .into_iter()
            .filter(|line| {
                (*line)
                    .clone()
                    .to::<[VectorType<T, N>; 2]>()
                    .into_iter()
                    .any(|vector| Rc::ptr_eq(&vector, &nearest_vector))
            })
            .collect::<Vec<LineType<T, N>>>()
    }
}

impl<T: Number, const N: usize> Nearest<VectorType<T, N>, LineType<T, N>> for Vec<LineType<T, N>> {
    fn nearest(&self, vector: &VectorType<T, N>) -> LineType<T, N> {
        let nearest_vector: VectorType<T, N> = self.nearest(vector);

        (*self)
            .clone()
            .into_iter()
            .filter(|line| line.has(&nearest_vector))
            .min_by(|ab, bc| {
                let [a, b] =
                    [ab, bc].map(|line| line.iter().find(|&vec| *vec != nearest_vector).unwrap());
                a.partial_cmp(&b).unwrap()
            })
            .unwrap()
    }
}
