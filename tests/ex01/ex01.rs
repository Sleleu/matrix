use matrix::colors::*;
use matrix::vector::Vector;

fn linear_combination_test() {

    println!("\n\n{GREEN}   LINEAR COMBINATION TESTS    {END}\n");

    println!("{YELLOW}Example tests{END}");
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{CYAN}{}{END}", Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{CYAN}{}{END}", Vector::linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]

    println!("{YELLOW}Other tests{END}");
    let u1 = Vector::from([1, 2, 3]);
    let u2 = Vector::from([4, 5, 6]);
    println!("{CYAN}{}{END}", Vector::linear_combination(&[u1, u2], &[10, 5]));

}

fn main() {
    linear_combination_test();
}