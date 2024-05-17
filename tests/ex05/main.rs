use matrix::colors::*;
use matrix::vector::Vector;

fn cosine_test() {

    println!("{GREEN}Example tests{END}");

    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("{:?}", Vector::angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("{:?}", Vector::angle_cos(&u, &v));
    // 0.0
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([ 1., -1.]);
    println!("{:?}", Vector::angle_cos(&u, &v));
    // -1.0
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("{:?}", Vector::angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{:?}", Vector::angle_cos(&u, &v));
    // 0.974631846
}

fn main() {
    cosine_test();
}