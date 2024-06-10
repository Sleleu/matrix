use crate::matrix::model::Matrix;
use std::ops::Add;

impl<K> Matrix<K>
where K: Add<Output = K> + Copy + Default {
    pub fn trace(&self) -> K {
        let mut trace_matrix: K = K::default();
        let n: usize = self.data[0].len();
        
        for i in 0 .. n {
            trace_matrix = trace_matrix + self.data[i][i];
        }
        trace_matrix
    }
}