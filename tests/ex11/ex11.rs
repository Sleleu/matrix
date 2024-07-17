use matrix::colors::*;
use matrix::matrix::Matrix;

fn determinant_test() {

    println!("\n\n{GREEN} DETERMINANT TESTS {END}\n");

    let u = Matrix::from([
    [ 1., -1.],
    [-1., 1.],
    ]);
    println!("{}", u.determinant());
    // 0.0

    let u = Matrix::from([
    [2., 0., 0.],
    [0., 2., 0.],
    [0., 0., 2.],
    ]);
    println!("{}", u.determinant());
    // 8.0

    let u = Matrix::from([
    [8., 5., -2.],
    [4., 7., 20.],
    [7., 6., 1.],
    ]);
    println!("{}", u.determinant());
    // -174.0
    
    let u = Matrix::from([
    [ 8., 5., -2., 4.],
    [ 4., 2.5, 20., 4.],
    [ 8., 5., 1., 4.],
    [28., -4., 17., 1.],
    ]);
    println!("{}", u.determinant());
    // 1032

    println!("\n\n{GREEN} OTHER TESTS {END}\n");

    println!("{CYAN}example from : https://fr.wikipedia.org/wiki/Calcul_du_d%C3%A9terminant_d%27une_matrice{END}");
    let u: Matrix<f32> = Matrix::from([
        [-2., 2., -3.],
        [-1., 1., 3.],
        [2., 0., -1.],
        ]);
    println!("{}", u.determinant());
    // 18

    println!("{CYAN}example from : http://maths.akkouche.free.fr/Maths/Fiches/Algebre_Fiche_2_Pivot_Gauss.pdf{END}");
    let u: Matrix<f32> = Matrix::from([
        [9., 3., -9.],
        [2., 1., 2.],
        [2., -1., -2.],
        ]);
    println!("{}", u.determinant());
    // 60

    println!("{CYAN}Last line with only 0{END}");
    let u: Matrix<f32> = Matrix::from([
        [9., 3., -9.],
        [0., 1., 2.],
        [0., 0., 0.],
        ]);
    println!("{}", u.determinant());
    // 0

    println!("{CYAN}Second column with only 0{END}");
    let u: Matrix<f32> = Matrix::from([
        [1., 0., -9.],
        [4., 0., 2.],
        [3., 0., 5.],
        ]);
    println!("{}", u.determinant());
    // 0

    println!("{CYAN}Undefined output{END}");
    let u: Matrix<i32> = Matrix::from([
        [42],
    ]);
    println!("{}", u.determinant());
    // undefined

    println!("{CYAN}Undefined output{END}");
    let u: Matrix<i32> = Matrix::from([
        [1,2,4,13,0,0],
    ]);
    println!("{}", u.determinant());
    // undefined

}

fn main() {
    determinant_test();
}