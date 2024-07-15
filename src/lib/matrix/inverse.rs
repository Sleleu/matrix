use crate::matrix::model::Matrix;
use std::ops::{Add, Sub, Mul, Div, Neg};

impl<K> Matrix<K>
where K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + PartialEq + Copy + Default + Neg<Output = K> {
    pub fn inverse(&self) -> Matrix<K> {
        let row: usize = self.data.len();
        let col: usize = self.data[0].len();
        let det: K = self.determinant(); // si det 0 pas d'inverse, a faire

        let inverse: Matrix<K> = Matrix { data: vec![vec![K::default(); row]; col] }; // utiliser inverse.row_echelon et recuperer la matrice identite transformee en matrice inverse

        inverse
    }
}