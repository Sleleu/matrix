use matrix::colors::*;
use matrix::matrix::Matrix;

fn row_echelon_form_test() {
    println!("\n\n{GREEN} ROW ECHELON FORM TESTS {END}\n");

    let u = Matrix::from([
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from([
        [1., 2.],
        [3., 4.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let u: Matrix<f64> = Matrix::from([
        [1., 2.],
        [2., 4.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let u: Matrix<f64> = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]

    println!("\n\n{GREEN} OTHER TESTS {END}\n");

    let u: Matrix<f64> = Matrix::from([
        [1., 1., 2., 3.],
        [1., 2., 1., 1.],
        [2., 1., 1., 0.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0][0.0][0.0]
    // [0.0][1.0][0.0]
    // [-0.0][-0.0][1.0]

    let u: Matrix<f32> = Matrix::from([
        [1., 1., 2., 3.],
        [1., 2., 1., 1.],
        [2., 1., 1., 0.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0][0.0][0.0]
    // [0.0][1.0][0.0]
    // [-0.0][-0.0][1.0]

    let u: Matrix<i32> = Matrix::from([
        [0]
    ]);
    println!("{}", u.row_echelon());
}

fn main() {
    row_echelon_form_test();
}
