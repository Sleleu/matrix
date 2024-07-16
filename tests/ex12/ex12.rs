use matrix::colors::*;
use matrix::matrix::Matrix;

fn inverse_test() {

    println!("\n\n{GREEN} INVERSE TESTS {END}\n");
    let mut u = Matrix::from([
    [1., 0., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
    ]);
    match u.inverse() {
        Ok(inverse) => println!("{}", inverse),
        Err(e) => eprintln!("{}\n",e),
    }
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let mut u = Matrix::from([
    [2., 0., 0.],
    [0., 2., 0.],
    [0., 0., 2.],
    ]);
    match u.inverse() {
        Ok(inverse) => println!("{}", inverse),
        Err(e) => eprintln!("{}\n",e),
    }
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    let mut u = Matrix::from([
    [8., 5., -2.],
    [4., 7., 20.],
    [7., 6., 1.],
    ]);
    match u.inverse() {
        Ok(inverse) => println!("{}", inverse),
        Err(e) => eprintln!("{}\n",e),
    }
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]

    println!("\n\n{GREEN} OTHER TESTS {END}\n"); 
    let mut u = Matrix::from([
        [2., 50., 6., 71., 8.],
        [4., 2., 0., 42., 1.,],
        [6., 12., 2., 1., 10.],
        [8., 1., 20., 1., 10.],
        [10., 0., 2., 1., 10.],
        ]);
        match u.inverse() {
            Ok(inverse) => println!("{}", inverse),
            Err(e) => eprintln!("{}\n",e),
        }
    // [0.2015356112160197][-0.33765632237146964][-0.7790376598852772][-0.05301599743470999][0.704590800584318]
    // [0.06717853707200651][-0.11255210745715649][-0.1763458866284256][-0.01767199914490332][0.15153026686143925]
    // [0.01866070474222396][-0.0312644742936543][-0.07676274628567364][0.05064666690419354][0.01431396301706631]
    // [-0.017547297538034065][0.05326540064844847][0.06382940820180306][0.004881177183168862][-0.05999928741938949]
    // [-0.203513022410661][0.33858267716535545][0.7880072683222311][0.04239854633555437][-0.601453664445792]

    let mut u = Matrix::from([
        [1., 1., 1., 1., 1.],
        [1., 1., 1., 1., 1.,],
        [1., 1., 1., 1., 1.],
        [1., 1., 1., 1., 1.],
        [1., 1., 0., 1., 1.],
        ]);
        match u.inverse() {
            Ok(inverse) => println!("{}", inverse),
            Err(e) => eprintln!("{}\n",e),
        }
    // The matrix is not invertible

    let mut u = Matrix::from([
        [8., 5., -2.],
        [0., 0., 0.],
        [7., 6., 1.],
        ]);
        match u.inverse() {
            Ok(inverse) => println!("{}", inverse),
            Err(e) => eprintln!("{}\n",e),
        }
    // The matrix is not invertible

    let mut u = Matrix::from([
        [8., 5.],
        [1., 0.],
        [7., 6.],
        ]);
        match u.inverse() {
            Ok(inverse) => println!("{}", inverse),
            Err(e) => eprintln!("{}\n",e),
        }
    // undefined

    let mut u = Matrix::from([
        [8., 1., 2.],
        [1., 0., 4.],
        ]);
        match u.inverse() {
            Ok(inverse) => println!("{}", inverse),
            Err(e) => eprintln!("{}\n",e),
        }
    // undefined

}

fn main() {
    inverse_test();
}