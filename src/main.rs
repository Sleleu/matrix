const GREEN: &str = "\x1b[1;32m";
const CYAN: &str = "\x1b[1;36m";
const END: &str = "\x1b[0m";
const YELLOW: &str = "\x1b[1;33m";

/* ---------------------------------------- VECTOR ---------------------------------------- */

use std::ops::{ Add,
                Sub,
                Mul};
use std::{fmt, usize};

struct Vector<K> {
    data: Vec<K>,
}

impl<K> Vector<K>
where K: Add<Output = K> + Copy {
    fn add(&mut self, v: &Vector<K>) {
        for (u_i, v_i) in self.data.iter_mut().zip(&v.data) {
            *u_i = *u_i + *v_i;
        }
    }
}

impl<K> Vector<K>
where K: Sub<Output = K> + Copy {
    fn sub(&mut self, v: &Vector<K>) {
        for (u_i, v_i) in self.data.iter_mut().zip(&v.data) {
            *u_i = *u_i - *v_i;
        }
    }
}

impl<K> Vector<K>
where K: Mul<Output = K> + Copy {
    fn scl(&mut self, k: &K) {
        for u_i in self.data.iter_mut() {
            *u_i = *u_i * *k;
        }
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K>
where K: Copy {
    fn from(item: [K; N]) -> Self {
        Vector {data: item.to_vec()}
    }
}

impl<K> fmt::Display for Vector<K>
where K: fmt::Display + std::fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, value) in self.data.iter().enumerate() {
            if i < self.data.len() - 1 {
                writeln!(f, "[{:?}]", value)?;
            }
            else {
                write!(f, "[{:?}]", value)?;
            }
        }
        Ok(())
    }
}

/* ---------------------------------------- MATRIX ---------------------------------------- */

struct Matrix<K> {
    data: Vec<Vec<K>>
}

impl<K> Matrix<K>
where K: Add<Output = K> + Copy {
    fn add(&mut self, v: &Matrix<K>) {
        for (row_a, row_b) in self.data.iter_mut().zip(&v.data) {
            for (a, b) in row_a.iter_mut().zip(row_b) {
                *a = *a + *b;
            }
        }
    }
}

impl<K> Matrix<K>
where K: Sub<Output = K> + Copy {
    fn sub(&mut self, v: &Matrix<K>) {
        for (row_a, row_b) in self.data.iter_mut().zip(&v.data) {
            for (a, b) in row_a.iter_mut().zip(row_b) {
                *a = *a - *b;
            }
        }
    }
}

impl<K> Matrix<K>
where K: Mul<Output = K> + Copy {
    fn scl(&mut self, k: &K) {
        for row_a in self.data.iter_mut() {
            for a in row_a.iter_mut() {
                *a = *a * *k;
            }
        }
    }
}


impl<K, const N: usize, const M: usize> From<[[K; M]; N]> for Matrix<K>
where K: Copy {
    fn from(item: [[K; M]; N]) -> Self {
        Matrix { data: item.iter().map(|&row| row.to_vec()).collect() }
    }
}

impl<K: fmt::Display + fmt::Debug> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for (i, value) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    write!(f, "[{:?}]", value)?;
                } else {
                    write!(f, "[{:?}]", value)?; 
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


/* ---------------------------------------- TESTS ---------------------------------------- */

fn vector_tests() {
    println!("{GREEN}============= ADD ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u: Vector<f64> = Vector::from([2.,3.]);
    let v: Vector<f64> = Vector::from([5., 7.]);
    u.add(&v);
    println!("{CYAN}{}{END}", u);
    // [7.0]
    // [10.0]

    println!("{GREEN}============= SUB ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u: Vector<f64> = Vector::from([2., 3.]);
    let v: Vector<f64> = Vector::from([5., 7.]);
    u.sub(&v);
    println!("{CYAN}{}{END}", u);
    // [-3.0]
    // [-4.0]

    println!("{GREEN}============= SCL ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u: Vector<f64> = Vector::from([2., 3.]);
    u.scl(&2.);
    println!("{CYAN}{}{END}", u);
    // [4.0]
    // [6.0]

}

fn matrix_tests() {

    println!("{GREEN}============= ADD ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut a: Matrix<f64> = Matrix::from([ [1., 2.],
                                            [3., 4.]]);
    let b: Matrix<f64> = Matrix::from([ [7., 4.],
                                        [-2., 2.]]);
    a.add(&b);
    println!("{CYAN}{}{END}", a);
    // [8.0, 6.0]
    // [1.0, 6.0]

    println!("{GREEN}============= SUB ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut a: Matrix<f64> = Matrix::from([ [1., 2.],
                                            [3., 4.]]);
    let b: Matrix<f64> = Matrix::from([ [7., 4.],
                                        [-2., 2.]]);
    a.sub(&b);
    println!("{CYAN}{}{END}", a);
    // [-6.0, -2.0]
    // [5.0, 2.0]


    println!("{GREEN}============= SCL ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut a: Matrix<f64> = Matrix::from([ [1., 2.],
                                            [3., 4.]]);
    a.scl(&2.);
    println!("{CYAN}{}{END}", a);
    // [2.0, 4.0]
    // [6.0, 8.0]
}

fn main() {
    println!("\n\n{GREEN}   RUN VECTOR TESTS    {END}\n");
    vector_tests();

    println!("\n\n{GREEN}   RUN MATRIX TESTS    {END}\n");
    matrix_tests();
}
