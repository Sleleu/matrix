use crate::matrix::model::Matrix;

impl<K> Matrix<K>
where K: Copy + Default {
    pub fn transpose(&self) -> Matrix<K> {
        let m: usize = self.data.len();
        let n: usize = self.data[0].len();
        let mut transposed_matrix: Matrix<K> = Matrix { data: vec![vec![K::default(); m]; n] };

        for i in 0 .. m {
            for j in 0 .. n {
                transposed_matrix.data[j][i] = self.data[i][j];
            }
        }
        transposed_matrix
    }
}