use std::fmt;
use crate::vector::model::Vector;

impl<K, const N: usize> From<[K; N]> for Vector<K>
where K: Copy {
    fn from(item: [K; N]) -> Self {
        Vector {data: item.to_vec()}
    }
}

impl<K> fmt::Display for Vector<K>
where K: fmt::Display + std::fmt::Debug + Copy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, value) in self.data.iter().enumerate() {
            if i < self.data.len() - 1 {
                writeln!(f, "[{:?}]", value)?;
            }
            else {
                write!(f, "[{:?}]", value)?;
            }
        }
        Ok(())
    }
}