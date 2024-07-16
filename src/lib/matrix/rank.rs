use crate::matrix::model::Matrix;
use std::ops::{Add, Div, Mul, Sub};

impl<K> Matrix<K>
where K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + Default + Clone + Copy + PartialEq
{
    pub fn rank(&self) -> usize {
        let row_echelon_matrix: Matrix<K> = self.row_echelon();
        let mut rank: usize = 0;

        for row in row_echelon_matrix.data.iter() {
            for i in row.iter() {
                if *i != K::default() {
                    rank += 1;
                    break;
                }
            }
        }
        rank
    }
}