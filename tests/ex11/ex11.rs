use matrix::colors::*;
use matrix::matrix::Matrix;

fn determinant_test() {
    let u = Matrix::from([
    [ 1., -1.],
    [-1., 1.],
    ]);
    println!("{}", u.determinant());
    // 0.0
    let u = Matrix::from([
    [2., 0., 0.],
    [0., 2., 0.],
    [0., 0., 2.],
    ]);
    println!("{}", u.determinant());
    // 8.0
    let u = Matrix::from([
    [8., 5., -2.],
    [4., 7., 20.],
    [7., 6., 1.],
    ]);
    println!("{}", u.determinant());
    // -174.0
    let u = Matrix::from([
    [ 8., 5., -2., 4.],
    [ 4., 2.5, 20., 4.],
    [ 8., 5., 1., 4.],
    [28., -4., 17., 1.],
    ]);
    println!("{}", u.determinant());
    // 1032
}

fn main() {
    determinant_test();
}