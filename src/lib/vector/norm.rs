use std::ops::{Add, Neg, Mul};
use std::cmp::PartialOrd;
use crate::vector::model::Vector;

fn abs_k<K>(x: K) -> K
where K: Neg<Output = K> + PartialOrd + Copy + Default {
    if x < K::default() {-x} else {x}
}

// manhattan
impl<K> Vector<K>
where K:    Neg<Output = K> + PartialOrd + Copy + Default,
      f64:  Add<K, Output = f64> {
    pub fn norm_1(&self) -> f64 {
        self.data.iter()
            .map(|&x| {
                abs_k(x)
            })
            .fold(0.0, |acc: f64, x: K| acc + x)
        }
}

//euclidienne
impl<K> Vector<K> 
where K:    Copy + Mul<Output = K> + Add<Output = K> + Default,
      f64:  From<K> {
    pub fn norm(&self) -> f64 {
            let square_sum: f64 = self.dot(self).into();
            square_sum.powf(0.5)
        }
}

//infinie
impl<K> Vector<K>
where K:    Neg<Output = K> + Copy + Default + PartialOrd + Into<f64> {
    pub fn norm_inf(&self) -> f64 {
        self.data.iter()
                 .map(|&x| {abs_k(x).into()})
                 .fold(0.0, f64::max)
    }
}
