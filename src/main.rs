const GREEN: &str = "\x1b[1;32m";
const CYAN: &str = "\x1b[1;36m";
const END: &str = "\x1b[0m";
const YELLOW: &str = "\x1b[1;33m";

mod vector;
mod matrix;

use vector::Vector;
use matrix::Matrix;

fn vector_tests() {
    println!("{GREEN}============= ADD ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u: Vector<f64> = Vector::from([2.,3., 5.]);
    let v: Vector<f64> = Vector::from([5., 7.]);
    u.add(&v);
    println!("{CYAN}{}{END}", u);
    // [7.0]
    // [10.0]

    println!("{GREEN}============= SUB ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u: Vector<f64> = Vector::from([2., 3.]);
    let v: Vector<f64> = Vector::from([5., 7.]);
    u.sub(&v);
    println!("{CYAN}{}{END}", u);
    // [-3.0]
    // [-4.0]

    println!("{GREEN}============= SCL ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u: Vector<f64> = Vector::from([2., 3.]);
    u.scl(2.);
    println!("{CYAN}{}{END}", u);
    // [4.0]
    // [6.0]

}

fn matrix_tests() {

    println!("{GREEN}============= ADD ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut a: Matrix<f64> = Matrix::from([ [1., 2.],
                                            [3., 4.]]);
    let b: Matrix<f64> = Matrix::from([ [7., 4.],
                                        [-2., 2.]]);
    a.add(&b);
    println!("{CYAN}{}{END}", a);
    // [8.0, 6.0]
    // [1.0, 6.0]

    println!("{GREEN}============= SUB ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut a: Matrix<f64> = Matrix::from([ [1., 2.],
                                            [3., 4.]]);
    let b: Matrix<f64> = Matrix::from([ [7., 4.],
                                        [-2., 2.]]);
    a.sub(&b);
    println!("{CYAN}{}{END}", a);
    // [-6.0, -2.0]
    // [5.0, 2.0]


    println!("{GREEN}============= SCL ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut a: Matrix<f64> = Matrix::from([ [1., 2.],
                                            [3., 4.]]);
    a.scl(&2.);
    println!("{CYAN}{}{END}", a);
    // [2.0, 4.0]
    // [6.0, 8.0]
}

fn linear_combination_test() {

    println!("\n\n{GREEN}   LINEAR COMBINATION TESTS    {END}\n");

    println!("{YELLOW}Example tests{END}");
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{CYAN}{}{END}", Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{CYAN}{}{END}", Vector::linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]

    println!("{YELLOW}Other tests{END}");
    let u1 = Vector::from([1, 2, 3]);
    let u2 = Vector::from([4, 5, 6]);
    println!("{CYAN}{}{END}", Vector::linear_combination(&[u1, u2], &[10, 5]));

}

fn main() {
    println!("\n\n{GREEN}   RUN VECTOR TESTS    {END}\n");
    vector_tests();

    println!("\n\n{GREEN}   RUN MATRIX TESTS    {END}\n");
    matrix_tests();

    linear_combination_test();
}
