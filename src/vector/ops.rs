use crate::vector::model::Vector;
use std::ops::{Add, Sub, Mul};

impl<K> Vector<K>
where K: Add<Output = K> + Copy {
    pub fn add(&mut self, v: &Vector<K>) {
        for (u_i, v_i) in self.data.iter_mut().zip(&v.data) {
            *u_i = *u_i + *v_i;
        }
    }
}

impl<K> Vector<K>
where K: Sub<Output = K> + Copy {
    pub fn sub(&mut self, v: &Vector<K>) {
        for (u_i, v_i) in self.data.iter_mut().zip(&v.data) {
            *u_i = *u_i - *v_i;
        }
    }
}

impl<K> Vector<K>
where K: Mul<Output = K> + Copy {
    pub fn scl(&mut self, k: K) {
        for u_i in self.data.iter_mut() {
            *u_i = *u_i * k;
        }
    }
}


// for usage of '+', '-' and '*' 
impl<K> Add for Vector<K>
where K: Add<Output = K> + Copy {
    type Output = Self;

    fn add(self, v: Vector<K>) -> Self::Output {
        Vector { data: self.data.iter().zip(v.data.iter()).map(|(&u_i, &v_i)| u_i + v_i).collect() }
    }
}

impl<K> Sub for Vector<K>
where K: Sub<Output = K> + Copy {
    type Output = Self;

    fn sub(self, v: Vector<K>) -> Self::Output {
        Vector { data: self.data.iter().zip(v.data.iter()).map(|(&u_i, &v_i)| u_i - v_i).collect() }
    }
}


impl<K> Mul<K> for Vector<K>
where K: Mul<Output = K> + Copy {
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        Vector { data: self.data.iter().map(|&u_i| u_i * scalar).collect() }
    }
}

impl<K> Vector::<K>
where K: Mul<Output = K> + Add<Output = K> + Copy + Default {
    pub fn dot(&self, v: &Vector::<K>) -> K {
        let mut dot_product = K::default();
        for (&u, v) in self.data.iter().zip(&v.data) {
            dot_product = dot_product + u * *v;
        }
        dot_product
    }
}

