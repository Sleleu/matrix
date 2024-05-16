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

impl<K> Add for Matrix<K>
where K: Add<Output = K> + Copy {
    type Output = Self;
    fn add(self, v: Matrix<K>) -> Self::Output {
        let matrix_add_result: Vec<Vec<K>> = self.data.iter().zip(v.data.iter()).map(|(row_a, row_b)| {
                                                row_a.iter().zip(row_b.iter()).map(|(&a, &b)| a + b).collect()
                                                }).collect();
        Matrix { data: matrix_add_result }
    }
}

impl<K> Sub for Matrix<K>
where K: Sub<Output = K> + Copy {
    type Output = Self;
    fn sub(self, v: Matrix<K>) -> Self::Output {
        let matrix_sub_result: Vec<Vec<K>> = self.data.iter().zip(v.data.iter()).map(|(row_a, row_b)| { 
                                                row_a.iter().zip(row_b.iter()).map(|(&a, &b)| a - b).collect()
                                                }).collect();
        Matrix { data: matrix_sub_result }
    }
}

impl<K> Mul<K> for Matrix<K>
where K: Mul<Output = K> + Copy {
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        let matrix_mul_result: Vec<Vec<K>> = self.data.iter()
                                                        .map(|row_a: &Vec<K>| {
                                                            row_a.iter().map(|&a_i | a_i * scalar).collect()
                                                        }).collect();
        Matrix { data: matrix_mul_result }
    }
}