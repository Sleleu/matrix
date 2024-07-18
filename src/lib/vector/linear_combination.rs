use crate::vector::model::Vector;
use std::ops::{AddAssign, Mul};

impl<K> Vector<K> 
where K: AddAssign + Mul<Output = K> + Copy + Default, {
    pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
        let zero = K::default();
        let vec_len = u[0].data.len();
        let mut u_result = Vector { data: vec![zero; vec_len] };

        for (u_vector_iterator, &coef_iterator) in u.iter().zip(coefs) {
            for (u_result_iterator, &u_vector_element_iterator)
                in u_result.data.iter_mut().zip(&u_vector_iterator.data) {
                    *u_result_iterator += u_vector_element_iterator * coef_iterator;
                }
        }
        u_result
    }
}
