use matrix::colors::*;
use matrix::vector::Vector;

fn dot_product_test() {

    println!("\n\n{GREEN}   DOT PRODUCT TESTS    {END}\n");

    println!("{YELLOW}Example tests{END}");
    let u: Vector<f64> = Vector::from([0., 0.]);
    let v: Vector<f64> = Vector::from([1., 1.]);
    println!("{:?}", u.dot(&v));
    // 0.0
    let u: Vector<f64> = Vector::from([1., 1.]);
    let v: Vector<f64> = Vector::from([1., 1.]);
    println!("{:?}", u.dot(&v));
    // 2.0
    let u: Vector<f64> = Vector::from([-1., 6.]);
    let v: Vector<f64> = Vector::from([3., 2.]);
    println!("{:?}", u.dot(&v));
    // 9.0

    println!("\n\n{GREEN} OTHER TESTS {END}\n");


    println!("{CYAN}Different size of vector : Result undefined{END}");
    let u: Vector<f64> = Vector::from([10., 2., 1., 10.]);
    let v: Vector<f64> = Vector::from([5., 1.]);
    println!("{:?}", u.dot(&v));
    // undefined

    println!("{CYAN}Empty vectors : result undefined{END}");
    let u: Vector<f64> = Vector::from([]);
    let v: Vector<f64> = Vector::from([]);
    println!("{:?}", u.dot(&v));
    // undefined

    println!("{CYAN}int vector:{END}");
    let u: Vector<i32> = Vector::from([3, 3]);
    let v: Vector<i32> = Vector::from([2, 2]);
    println!("{:?}", u.dot(&v));
    // 12

}

fn main() {
    dot_product_test();
}