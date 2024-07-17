use std::fmt;
use std::ops::{Add, Sub, Mul};
use crate::matrix::model::Matrix;

impl<K, const N: usize, const M: usize> From<[[K; M]; N]> for Matrix<K>
where K: Copy {
    fn from(item: [[K; M]; N]) -> Self {
        Matrix { data: item.iter().map(|&row| row.to_vec()).collect() }
    }
}

impl<K: fmt::Display + fmt::Debug> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for (i, value) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    write!(f, "[{:?}]", value)?;
                } else {
                    write!(f, "[{:?}]", value)?; 
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<K> Add for Matrix<K>
where K: Add<Output = K> + Copy + Default {
    type Output = Self;
    fn add(mut self, v: Matrix<K>) -> Self::Output {
        for (row_a, row_b) in self.data.iter_mut().zip(&v.data) {
            for (a, b) in row_a.iter_mut().zip(row_b) {
                *a = *a + *b;
            }
        }
        self
    }
}

impl<K> Sub for Matrix<K>
where K: Sub<Output = K> + Copy {
    type Output = Self;
    fn sub(mut self, v: Matrix<K>) -> Self::Output {
        for (row_a, row_b) in self.data.iter_mut().zip(&v.data) {
            for (a, b) in row_a.iter_mut().zip(row_b) {
                *a = *a - *b;
            }
        }
        self
    }
}

impl<K> Mul<K> for Matrix<K>
where K: Mul<Output = K> + Copy {
    type Output = Self;

    fn mul(mut self, scalar: K) -> Self::Output {
        for row_a in self.data.iter_mut() {
            for a in row_a.iter_mut() {
                *a = *a * scalar;
            }
        }
        self
    }
}