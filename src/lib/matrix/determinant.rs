use crate::matrix::model::Matrix;
use std::ops::{Add, Mul};

impl<K> Matrix<K>
where K: Add<Output = K> + Mul<Output = K> + Copy + Default {
    fn determinant(&mut self) -> K {

    }
}