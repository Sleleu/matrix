use matrix::colors::*;
use matrix::matrix::Matrix;
use matrix::vector::Vector;

fn matrix_multiplication_test() {

    println!("\n\n{GREEN} MUL VEC TESTS {END}\n");

    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [2.]
    let u = Matrix::from([
        [2., 0.],
        [0., 2.],
        ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [8.]
    // [4.]
    let u = Matrix::from([
        [2., -2.],
        [-2., 2.],
        ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [-4.]

    println!("\n\n{GREEN} MUL MAT TESTS {END}\n");

    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
    let v = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
    println!("{}", u.mul_mat(&v));
    // [1., 0.]
    // [0., 1.]
    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
    let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        ]);
    println!("{}", u.mul_mat(&v));
    // [2., 1.]
    // [4., 2.]
    let u = Matrix::from([
        [3., -5.],
        [6., 8.],
        ]);
    let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        ]);
    println!("{}", u.mul_mat(&v));
    // [-14., -7.]
    // [44., 22.]

    println!("\n\n{GREEN} OTHER TESTS {END}\n");

    println!("{CYAN}3*2 A | 2*3 B => 3*3 C{END}");
    let u = Matrix::from([
        [2., 1.],
        [4., 2.],
        [6., 5.],
        ]);
    let v = Matrix::from([
        [1., 0., 1.],
        [0., 1., 0.],
        ]);
    println!("{}", u.mul_mat(&v));
    // [2.0][1.0][2.0]
    // [4.0][2.0][4.0]
    // [6.0][5.0][6.0]

    println!("{CYAN}2*3 A | 3*2 B => 2*2 C{END}");
    let u = Matrix::from([
        [1., 0., 1.],
        [0., 1., 0.],
        ]);
    let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        [6., 5.],
        ]);
    println!("{}", u.mul_mat(&v));
    // [8.0][6.0]
    // [4.0][2.0]

    println!("{CYAN}4*3 A | 3*2 B => 4*2 C{END}");
    let u = Matrix::from([
        [1., 0., 1.],
        [0., 1., 0.],
        [1., 0., 1.],
        [0., 1., 0.],
        ]);
    let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        [6., 5.],
        ]);
    println!("{}", u.mul_mat(&v));
    // [8.0][6.0]
    // [4.0][2.0]
    // [8.0][6.0]
    // [4.0][2.0]

    println!("{CYAN}4*2 A | 3*2 : undefined{END}");
    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
        [1., 0.],
        [0., 1.],
        ]);
    let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        [6., 5.],
        ]);
    println!("{}", u.mul_mat(&v));
    // undefined

    println!("{CYAN}different size between A cols and B rows : undefined{END}");
    let u = Matrix::from([
        [1.],
        ]);
    let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        [6., 5.],
        ]);
    println!("{}", v.mul_mat(&u));
    // undefined

}

fn main() {
    matrix_multiplication_test();
}