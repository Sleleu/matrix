use crate::matrix::model::Matrix;

impl<K> Matrix<K>
where K: Copy + Default {
    pub fn transpose(&self) -> Matrix<K> {
        let mut transposed_matrix: Matrix<K> = Matrix { data: vec![vec![K::default(); self.row()]; self.col()] };

        for i in 0 .. self.row() {
            for j in 0 .. self.col() {
                transposed_matrix.data[j][i] = self.data[i][j];
            }
        }
        transposed_matrix
    }
}