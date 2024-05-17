use std::ops::{Add, Neg};
use std::cmp::PartialOrd;
use crate::vector::model::Vector;

fn abs_k<K>(x: K) -> K
where K: Neg<Output = K> + PartialOrd + Copy + Default {
    if x < K::default() {-x} else {x}
}

// manhattan
impl<K> Vector<K>
where K:    Neg<Output = K> + PartialOrd + Copy + Default,
      f32:  Add<K, Output = f32> {
    pub fn norm_1(&self) -> f32 {
        self.data.iter().map(|&x| abs_k(x)).fold(0.0, |acc: f32, x: K| acc + x)
    }
}

//euclidienne
