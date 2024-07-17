use crate::matrix::model::Matrix;
use std::ops::AddAssign;

impl<K> Matrix<K>
where K: AddAssign + Copy + Default {
    pub fn trace(&self) -> K {
        let mut trace_matrix: K = K::default();
        
        if self.col() != self.row() { 
            return trace_matrix 
        }
        for i in 0 .. self.col() {
            trace_matrix += self.data[i][i];
        }
        trace_matrix
    }
}