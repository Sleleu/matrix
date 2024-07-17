use matrix::colors::*;
use matrix::matrix::Matrix;

fn transpose_test() {
    println!("\n\n{GREEN} TRANSPOSE TESTS {END}\n");

    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
    ]);
    println!("Original:\n{}", u);
    println!("Transposed:\n{}", u.transpose());
    // 1. 0.
    // 0. 1.

    let u = Matrix::from([
        [2., -5., 0.],
        [4., 3., 7.],
        [-2., 3., 4.],
    ]);
    println!("Original:\n{}", u);
    println!("Transposed:\n{}", u.transpose());
    // 2. 4. -2.
    // -5. 3. 3.
    // 0. 7. 4.

    let u = Matrix::from([
        [-2., -8., 4.],
        [1., -23., 4.],
        [0., 6., 4.],
    ]);
    println!("Original:\n{}", u);
    println!("Transposed:\n{}", u.transpose());
    // -2. 1. 0.
    // -8. -23. 6.
    // 4. 4. 4.

    println!("\n{GREEN} OTHER TESTS {END}\n");

    println!("{CYAN}3x2 matrix{END}");
    let u = Matrix::from([
        [1., 4.],
        [2., 5.],
        [3., 6.],
    ]);
    println!("Original:\n{}", u);
    println!("Transposed:\n{}", u.transpose());
    // 1.0 2.0 3.0
    // 4.0 5.0 6.0

    println!("{CYAN}2x4 matrix{END}");
    let u = Matrix::from([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
    ]);
    println!("Original:\n{}", u);
    println!("Transposed:\n{}", u.transpose());
    // 1 5
    // 2 6
    // 3 7
    // 4 8

    println!("{CYAN}1x1 matrix{END}");
    let u = Matrix::from([
        [1.],
    ]);
    println!("Original:\n{}", u);
    println!("Transposed:\n{}", u.transpose());
    // 1


}

fn main() {
    transpose_test();
}
