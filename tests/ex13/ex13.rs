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

    println!("\n\n{GREEN} OTHER TESTS {END}\n");

    println!("\n{CYAN}Second column consists of zeros :{END}");
    let u = Matrix::from([
        [ 8., 0., -2.],
        [ 4., 0., 20.],
        [ 7., 0., 1.],
        [ 1., 0., 7.],
        ]);
    println!("{}", u.rank());
    // 2

    println!("\n{CYAN}Only zeros :{END}");
    let u = Matrix::from([
        [ 0., 0., 0.],
        [ 0., 0., 0.],
        [ 0., 0., 0.],
        [ 0., 0., 0.],
        ]);
    println!("{}", u.rank());
    // 0


    println!("\n{CYAN}Same example as wikipedia (https://fr.wikipedia.org/wiki/Rang_(alg%C3%A8bre_lin%C3%A9aire)) :{END}");
    let u = Matrix::from([
        [ 1., 0., 2., 3.],
        [ 2., 0., 4., 6.],
        [ 0., 2., 2., 0.],
        [ 1., 2., 4., 3.],
        ]);
    println!("{}", u.rank());
    // 2

    println!("\n{CYAN}Random values, the system has a unique solution :{END}");
    let u = Matrix::from([
        [ 15345., 0.4, 23123134535.44475, -3213123355345.73501, 5.],
        [ 265433., 0.1, 4453453453131.47575, 631235543543.124770477, 4.],
        [ 0.1, 2343443.41404, 23213455345., 01.4775140, 8.],
        [ 11016894414.41014, -2972101.4140411, 4435389., -3312344.0107575404, 3.],
        ]);
    println!("{}", u.rank());
    // 4

    println!("\n{CYAN}(l1, l2, l3) are linearly dependant :{END}");
    let u = Matrix::from([
        [1., 2., 8.],
        [2., 4., 16.],
        [3., 6., 24.],
        ]);
    println!("{}", u.rank());
    // 1


}

fn main () {
    rank_test();
}