use crate::vector::model::Vector;
use std::ops::{Add, Mul};

impl<K> Vector<K>
where K: Mul<Output = K> + Add<Output = K> + Copy + Default {
    pub fn dot(&self, v: &Vector::<K>) -> K {
        let mut dot_product = K::default();
        for (&u, v) in self.data.iter().zip(&v.data) {
            dot_product = dot_product + (u * *v);
        }
        dot_product
    }
}
