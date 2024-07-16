#[derive(Clone)]
pub struct Matrix<K> {
    pub data: Vec<Vec<K>>
}

impl <K> Matrix<K>  {
    pub fn row(&self) -> usize {
        self.data.len()
    }

    pub fn col(&self) -> usize {
        self.data[0].len()
    }
}