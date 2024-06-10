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
}

fn main() {
    transpose_test();
}
