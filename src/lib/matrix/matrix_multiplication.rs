use crate::matrix::model::Matrix;
use crate::vector::model::Vector;
use std::ops::{Add, Mul};

impl<K> Matrix<K>
where K: Add<Output = K> + Mul<Output = K> + Copy + Default {
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        Vector {
            data: self.data.iter().map(|row: &Vec<K>| {row.iter().zip(&vec.data)
                .fold(K::default(), |acc, (&a_i, &u_i)| acc + a_i * u_i)
            }).collect(),
        }
    }
}

impl<K> Matrix<K>
where K: Add<Output = K> + Mul<Output = K> + Copy + Default {
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let m = self.row();
        let n = mat.col();
        let p = self.col();

        let mut result =  Matrix { data: vec![vec![K::default(); n]; m] };

        if self.col() != mat.row() {
            println!("undefined");
            return result;
        }

        for i in 0..m {
            for j in 0..n {
                let mut sum = K::default();
                for k in 0..p {
                    sum = sum + self.data[i][k] * mat.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }

        result
    }
}
