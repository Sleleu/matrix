use crate::vector::model::Vector;
use std::ops::{Add, Mul};

impl<K> Vector<K> 
where K: Copy + Default + Add<Output = K> + Mul<Output = K> + Into<f64>,
      f64: From<K> {
    pub fn angle_cos(u: &Vector::<K>, v: &Vector::<K>) -> f64 {
        let dot_product: f64 = u.dot(v).into();
        let u_norm = u.norm();
        let v_norm = v.norm();
        dot_product / (u_norm * v_norm)
    }
}