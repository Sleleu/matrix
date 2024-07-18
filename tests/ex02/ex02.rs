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

    println!("\n\n{GREEN} OTHER TESTS {END}\n");

    let u: Vector<f32> = Vector::from([10., 10.]);
    let v: Vector<f32> = Vector::from([-10., 20.]);
    let res: Vector<f32> = lerp(u, v, 0.5);
    println!("{CYAN}Distance between 10 -10 and between 10 20, temperature : 0.5{END}");
    println!("{}", res);
    // [0.0]
    // [15.0]

    let a: Matrix<f32> = Matrix::from([[0., 1., 2.], 
                                       [3., 4., 5.],
                                       [6., 7., 8.]]);

    let b: Matrix<f32> = Matrix::from([[2., 3., 4.], 
                                       [5., 6., 7.],
                                       [8., 9., 10.]]);
    let c: Matrix<f32> = lerp(a, b, 0.5);
    println!("{CYAN}With Aii | Aii+1 matrix :{END}"); 
    println!("{}", c);
    // [1.0][2.0][3.0]
    // [4.0][5.0][6.0]
    // [7.0][8.0][9.0]

    let a: Matrix<f32> = Matrix::from([[0., 0., 0.], 
                                       [0., 0., 0.],
                                       [0., 0., 0.]]);

    let b: Matrix<f32> = Matrix::from([[0., 0., 0.], 
                                       [0., 0., 0.],
                                       [0., 0., 0.]]);
    let c: Matrix<f32> = lerp(a, b, 1.);
    println!("{CYAN}A and B are zero matrix :{END}"); 
    println!("{}", c);
    // [0.0][0.0][0.0]
    // [0.0][0.0][0.0]
    // [0.0][0.0][0.0]
    let a: Matrix<f32> = Matrix::from([[10., 5., 42.], 
                                       [5., 6., 2.],
                                       [1., 1., 2.],
                                       [2., 8., 7.]]);

    let b: Matrix<f32> = Matrix::from([[20., 2.], 
                                       [3., 5.],
                                       [1., 8.]]);
    let c: Matrix<f32> = lerp(a, b, 0.5);
    println!("{CYAN}A = 4x3 matrix and B = 3x2 matrix ? :{END}"); 
    println!("{}", c);
    // [15.0][3.5][42.0]
    // [4.0][5.5][2.0]
    // [1.0][4.5][2.0]
    // [2.0][8.0][7.0]

    let u: Vector<f32> = Vector::from([1., 2.]);
    let v: Vector<f32> = Vector::from([-1., 1.]);
    let res: Vector<f32> = lerp(u, v, 0.5);
    println!("{CYAN}test{END}");
    println!("{}", res);
    // [0.0]
    // [15.0]

}

fn main() {
    linear_interpolation_test();
}