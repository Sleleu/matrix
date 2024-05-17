use matrix::colors::*;
use matrix::matrix::Matrix;
use matrix::vector::Vector;

pub fn vector_tests() {
    println!("{GREEN}============= ADD ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u: Vector<f64> = Vector::from([2.,3.]);
    let v: Vector<f64> = Vector::from([5., 7.]);
    u.add(&v);
    println!("{CYAN}{}{END}", u);
    // [7.0]
    // [10.0]

    println!("{YELLOW}Use of trait Add tests{END}");
    let res = u + v;
    println!("{CYAN}{}{END}", res);

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

    println!("{YELLOW}Use of trait Mul tests{END}");
    let res = u * 2.;
    println!("{CYAN}{}{END}", res);

}

pub fn matrix_tests() {

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
