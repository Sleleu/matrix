use matrix::colors::*;
use matrix::matrix::Matrix;

fn inverse_test() {

    println!("\n\n{GREEN} DETERMINANT TESTS {END}\n");
    let u = Matrix::from([
    [1., 0., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
    ]);
    println!("{}", u.inverse());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from([
    [2., 0., 0.],
    [0., 2., 0.],
    [0., 0., 2.],
    ]);
    println!("{}", u.inverse());
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    let u = Matrix::from([
    [8., 5., -2.],
    [4., 7., 20.],
    [7., 6., 1.],
    ]);
    println!("{}", u.inverse());
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]


}

fn main() {
    inverse_test();
}