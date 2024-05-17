use std::ops::{Add, Sub, Mul};

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where V: Add<Output = V> + Sub<Output = V> + Mul<f32, Output = V> + Clone {  
    u.clone() + (v - u) * t
}
