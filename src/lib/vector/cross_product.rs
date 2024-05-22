use crate::vector::model::Vector;
use std::ops::{Add, Sub, Mul};

impl<K> Vector<K> 
where K: Mul<Output = K> + Sub<Output = K> + Add<Output = K> + Copy {
    pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
        Vector {
            data: vec![
                (u.data[1] * v.data[2]) - (u.data[2] * v.data[1]),
                (u.data[2] * v.data[0]) - (u.data[0] * v.data[2]),
                (u.data[0] * v.data[1]) - (u.data[1] * v.data[0]),
            ],
        }
    }
}