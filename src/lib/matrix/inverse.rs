use crate::matrix::model::Matrix;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::convert::From;

impl<K> Matrix<K>
where K: From<i32> + std::ops::SubAssign + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + PartialEq + Copy + Default + Neg<Output = K> {
    
    fn get_identity(&self, col: usize, row: usize) -> Matrix<K> {

        let mut identity : Matrix<K> = Matrix { data: vec![vec![K::default(); col]; row] };

        for i in 0 .. col {
            identity.data[i][i] = 1.into()
        }
        identity
    }
    
    pub fn inverse(&mut self) -> Result<Matrix<K>, &str> {
        let mut pivot_index: usize = 0;
        let row: usize = self.data.len();
        let col: usize = self.data[0].len();
        let det: K = self.determinant(); // si det 0 pas d'inverse, a faire

        if det == K::default() {
            return Err("The matrix is not invertible");
        }

        let mut identity: Matrix<K> = self.get_identity(col, row);

        for r in 0..row {
            if pivot_index >= col {
                break;
            }

            let mut pivot_row: usize = r;
            while self.data[pivot_row][pivot_index] == K::default() {
                pivot_row += 1;
                if pivot_row == row {
                    pivot_row = r;
                    pivot_index += 1; 
                    if pivot_index == col {
                        break;
                    }
                }
            }

            if pivot_index == col {
                break;
            }

            self.data.swap(r, pivot_row);
            identity.data.swap(r, pivot_row);

            let pivot_value = self.data[r][pivot_index];
            if pivot_value != K::default() {
                for j in 0..col {
                    identity.data[r][j] = identity.data[r][j] / pivot_value;
                    self.data[r][j] = self.data[r][j] / pivot_value;
                }
            }

            for i in 0..row {
                if i != r && self.data[i][pivot_index] != K::default() {
                    let factor: K = self.data[i][pivot_index];
                    for k in 0..col {
                        self.data[i][k] = self.data[i][k] - factor * self.data[r][k];
                        identity.data[i][k] =  identity.data[i][k] - factor * identity.data[r][k];
                    }
                }
            }
            pivot_index += 1;
        }

        Ok(identity)
    }
}