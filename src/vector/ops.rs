use crate::vector::model::Vector;
use std::ops::{Add, Sub, Mul};

impl<K> Vector<K>
where K: Add<Output = K> + Copy {
    pub fn add(&mut self, v: &Vector<K>) {
        for (u_i, v_i) in self.data.iter_mut().zip(&v.data) {
            *u_i = *u_i + *v_i;
        }
    }
}

impl<K> Vector<K>
where K: Sub<Output = K> + Copy {
    pub fn sub(&mut self, v: &Vector<K>) {
        for (u_i, v_i) in self.data.iter_mut().zip(&v.data) {
            *u_i = *u_i - *v_i;
        }
    }
}

impl<K> Vector<K>
where K: Mul<Output = K> + Copy {
    pub fn scl(&mut self, k: K) {
        for u_i in self.data.iter_mut() {
            *u_i = *u_i * k;
        }
    }
}