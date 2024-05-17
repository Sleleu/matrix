use std::fmt;
use crate::matrix::model::Matrix;

impl<K, const N: usize, const M: usize> From<[[K; M]; N]> for Matrix<K>
where K: Copy {
    fn from(item: [[K; M]; N]) -> Self {
        Matrix { data: item.iter().map(|&row| row.to_vec()).collect() }
    }
}

impl<K: fmt::Display + fmt::Debug> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for (i, value) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    write!(f, "[{:?}]", value)?;
                } else {
                    write!(f, "[{:?}]", value)?; 
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}