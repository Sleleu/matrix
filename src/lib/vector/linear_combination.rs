use crate::vector::model::Vector;
use std::ops::{Add, Mul};

impl<K> Vector<K> 
where K: Add<Output = K> + Mul<Output = K> + Copy + Default, {
    pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
        let zero = K::default();
        let vec_len = u[0].data.len();
        let mut u_result = Vector { data: vec![zero; vec_len] };

        for (v, &c_i) in u.iter().zip(coefs) {
            for (v_result_i, &u_i) in u_result.data.iter_mut().zip(&v.data) {
                *v_result_i = *v_result_i + u_i * c_i;
            }
        }
        u_result
    }
}
