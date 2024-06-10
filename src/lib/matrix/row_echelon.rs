use crate::matrix::model::Matrix;
use std::ops::Add;

impl<K> Matrix<K>
where K: Add<Output = K> + Copy + Default {
    fn row_echelon(&self) -> Matrix<K> {

    }
}