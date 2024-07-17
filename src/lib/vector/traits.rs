use std::fmt;
use std::ops::{Add, Sub, Mul};
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

impl<K> Add for Vector<K>
where K: Add<Output = K> + Copy {
    type Output = Self;

    fn add(mut self, v: Vector<K>) -> Self::Output {
        for (u_i, v_i) in self.data.iter_mut().zip(&v.data) {
            *u_i = *u_i + *v_i;
        }
        self
    }
}

impl<K> Sub for Vector<K>
where K: Sub<Output = K> + Copy {
    type Output = Self;

    fn sub(mut self, v: Vector<K>) -> Self::Output {
        for (u_i, v_i) in self.data.iter_mut().zip(&v.data) {
            *u_i = *u_i - *v_i;
        }
        self
    }
}

impl<K> Mul<K> for Vector<K>
where K: Mul<Output = K> + Copy {
    type Output = Self;

    fn mul(mut self, scalar: K) -> Self::Output {
        for u_i in self.data.iter_mut() {
            *u_i = *u_i * scalar;
        }
        self
    }
}
