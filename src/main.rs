use std::ops::Add;
use std::fmt;

struct Vector<K> {
    data: Vec<K>,
}


// add
impl<K> Vector<K> where K: Add<Output = K> + Copy {
    fn add(&mut self, v: &Vector<K>) {
        for (a, b) in self.data.iter_mut().zip(&v.data) {
            *a = *a + *b;
        }
    }
}

// From trait
impl<K, const N: usize> From<[K; N]> for Vector<K> where K: Copy {
    fn from(item: [K; N]) -> Self {
        Vector {data: item.to_vec()}
    }
}

// Display trait
impl<K: fmt::Display + std::fmt::Debug> fmt::Display for Vector<K> {
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

fn main() {
    let mut u = Vector::from([2.,3.,5.]);
    let v = Vector::from([5., 7., 5.]);
    u.add(&v);
    println!("{}", u);
    // [7.0]
    // [10.0]
}
