use matrix::colors::*;
use matrix::vector::Vector;

fn linear_combination_test() {

    println!("\n\n{GREEN}   LINEAR COMBINATION TESTS    {END}\n");

    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{}", Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", Vector::linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]

    println!("\n\n{GREEN} OTHER TESTS {END}\n");

    println!("{CYAN} Simple test{END}");
    let u1 = Vector::from([1, 2, 3]);
    let u2 = Vector::from([4, 5, 6]);
    println!("{}", Vector::linear_combination(&[u1, u2], &[10, 5]));

    println!("{CYAN} Not the same size between Vec array and vec of scalars : Result undefined{END}");
    let u1 = Vector::from([1, 2]);
    let u2 = Vector::from([1, 2]);
    let u3 = Vector::from([1, 2]);
    let u4 = Vector::from([1, 2]);
    println!("{}", Vector::linear_combination(&[u1, u2, u3, u4], &[2, 4]));

    println!("{CYAN} Not the same size between Vec in the array : Result undefined{END}");
    let u1 = Vector::from([1]);
    let u2 = Vector::from([1, 2]);
    let u3 = Vector::from([1, 2, 3]);
    let u4 = Vector::from([1, 2, 3, 4]);
    println!("{}", Vector::linear_combination(&[u1, u2, u3, u4], &[2, 4, 6, 8]));

}

fn main() {
    linear_combination_test();
}