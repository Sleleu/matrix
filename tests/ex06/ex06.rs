use matrix::colors::*;
use matrix::vector::Vector;

fn cross_product_test() {

    println!("{GREEN}Example tests{END}");
    
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}

fn main() {
    cross_product_test();
}