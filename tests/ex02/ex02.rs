use matrix::colors::*;
use matrix::vector::Vector;
use matrix::matrix::Matrix;
use matrix::linear_interpolation::lerp;

fn linear_interpolation_test() {

    println!("\n\n{GREEN}   LINEAR INTERPOLATION TESTS    {END}\n");

    println!("{YELLOW}Example tests{END}");
    println!("{CYAN}{}{END}", lerp(0., 1., 0.));
    // 0.0
    println!("{CYAN}{}{END}", lerp(0., 1., 1.));
    // 1.0
    println!("{CYAN}{}{END}", lerp(0., 1., 0.5));
    // 0.5
    println!("{CYAN}{}{END}", lerp(21., 42., 0.3));
    // 27.3
    println!("{CYAN}{}{END}", lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));
    // [2.6]
    // [1.3]
    println!("{CYAN}{}{END}", lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20., 10.], [30., 40.]]), 0.5));
    // [[11., 5.5]
    // [16.5, 22.]]

    println!("{YELLOW}Other tests{END}");
    let u: Vector<f32> = Vector::from([10., 10.]);
    let v: Vector<f32> = Vector::from([-10., 20.]);
    let res: Vector<f32> = lerp(u, v, 0.5);
    println!("{CYAN}{}{END}", res);

    let a: Matrix<f32> = Matrix::from([[0., 1., 2.], 
                                       [3., 4., 5.],
                                       [6., 7., 8.]]);

    let b: Matrix<f32> = Matrix::from([[2., 3., 4.], 
                                       [5., 6., 7.],
                                       [8., 9., 10.]]);
    let c: Matrix<f32> = lerp(a, b, 0.5);
    println!("{CYAN}\nMatrix test \n[{}]{END}", c);
}

fn main() {
    linear_interpolation_test();
}