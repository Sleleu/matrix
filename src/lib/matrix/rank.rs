use crate::matrix::model::Matrix;
use std::{ops::{Add, Div, Mul, Sub}, usize};

impl<K> Matrix<K>
where K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + Default
{
    pub fn rank(&self) -> usize {
        let rank: usize = 42;

        rank
    }
}