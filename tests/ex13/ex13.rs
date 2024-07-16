use matrix::colors::*;
use matrix::matrix::Matrix;

fn rank_test() {

    println!("\n\n{GREEN} RANK TESTS {END}\n");

    let u = Matrix::from([
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        ]);
    println!("{}", u.rank());
    // 3
    let u = Matrix::from([
        [ 1., 2., 0., 0.],
        [ 2., 4., 0., 0.],
        [-1., 2., 1., 1.],
        ]);
    println!("{}", u.rank());
    // 2
    let u = Matrix::from([
        [ 8., 5., -2.],
        [ 4., 7., 20.],
        [ 7., 6., 1.],
        [21., 18., 7.],
        ]);
    println!("{}", u.rank());
    // 3
}

fn main () {
    rank_test();
}