use matrix::colors::*;
use matrix::matrix::Matrix;

fn trace_test() {

    println!("\n\n{GREEN} MATRIX TRACE TESTS {END}\n");

    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
    println!("{}", u.trace());
    // 2.0
    let u = Matrix::from([
        [2., -5., 0.],
        [4., 3., 7.],
        [-2., 3., 4.],
        ]);
    println!("{}", u.trace());
    // 9.0
    let u = Matrix::from([
        [-2., -8., 4.],
        [1., -23., 4.],
        [0., 6., 4.],
        ]);
    println!("{}", u.trace());
    // -21.0

    println!("\n{GREEN} OTHER TESTS {END}\n");

    println!("{CYAN}Simple 3x3 int matrix with only 10 on diagonal{END}");
    let u: Matrix<i32> = Matrix::from([
        [10, 0, 0],
        [0, 10, 0],
        [0, 0, 10],
        ]);
    println!("{}", u.trace());
    // 30

    println!("{CYAN}2x3 matrix: undefined result{END}");
    let u: Matrix<i32> = Matrix::from([
        [1, 0, 0],
        [0, 1, 0],
        ]);
    println!("{}", u.trace());
    // undefined

    println!("{CYAN}5x1 matrix: undefined result{END}");
    let u: Matrix<i32> = Matrix::from([
        [1],
        [1],
        [1],
        [1],
        [1],
        ]);
    println!("{}", u.trace());
    // undefined

    println!("{CYAN}1x1 matrix{END}");
    let u: Matrix<i32> = Matrix::from([
        [42],
        ]);
    println!("{}", u.trace());
    // 42

    println!("{CYAN}Zero matrix{END}");
    let u: Matrix<i32> = Matrix::from([
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        ]);
    println!("{}", u.trace());
    // 0

}

fn main() {
    trace_test();
}