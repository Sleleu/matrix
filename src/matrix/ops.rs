use crate::matrix::model::Matrix;
use std::ops::{Add, Sub, Mul};

impl<K> Matrix<K>
where K: Add<Output = K> + Copy {
    pub fn add(&mut self, v: &Matrix<K>) {
        for (row_a, row_b) in self.data.iter_mut().zip(&v.data) {
            for (a, b) in row_a.iter_mut().zip(row_b) {
                *a = *a + *b;
            }
        }
    }
}

impl<K> Matrix<K>
where K: Sub<Output = K> + Copy {
    pub fn sub(&mut self, v: &Matrix<K>) {
        for (row_a, row_b) in self.data.iter_mut().zip(&v.data) {
            for (a, b) in row_a.iter_mut().zip(row_b) {
                *a = *a - *b;
            }
        }
    }
}

impl<K> Matrix<K>
where K: Mul<Output = K> + Copy {
    pub fn scl(&mut self, k: &K) {
        for row_a in self.data.iter_mut() {
            for a in row_a.iter_mut() {
                *a = *a * *k;
            }
        }
    }
}