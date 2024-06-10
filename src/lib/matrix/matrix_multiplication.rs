use crate::matrix::model::Matrix;
use crate::vector::model::Vector;
use std::ops::{Add, Mul};

impl<K> Matrix<K>
where K: Add<Output = K> + Mul<Output = K> + Copy + Default {
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        Vector {
            data: self.data.iter()
            .map(|row: &Vec<K>| {
                row.iter()
                .zip(&vec.data)
                .fold(K::default(), |acc, (&a_i, &u_i)| acc + a_i * u_i)
            }).collect(),
        }
    }
}

impl<K> Matrix<K>
where K: Add<Output = K> + Mul<Output = K> + Copy + Default {
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let col_len: usize = mat.data[0].len();
        Matrix {
            data: self.data.iter()
            .map(|row_a: &Vec<K>| {
                (0 .. col_len)
                .map(|col_b: usize| {
                    row_a.iter()
                    .zip(mat.data.iter().map(|row_b| &row_b[col_b]))
                    .fold(K::default(), |acc, (&a, &b)| acc + a * b)
                }).collect()
            }).collect()
        }
    }
}
