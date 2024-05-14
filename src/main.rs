const GREEN: &str = "\x1b[1;32m";
const CYAN: &str = "\x1b[1;36m";
const END: &str = "\x1b[0m";
const YELLOW: &str = "\x1b[1;33m";

/* VECTOR */

use std::ops::{ Add,
                Sub,
                Mul};
use std::fmt;

struct Vector<K> {
    data: Vec<K>,
}

// add
impl<K> Vector<K>
where K: Add<Output = K> + Copy {
    fn add(&mut self, v: &Vector<K>) {
        for (a, b) in self.data.iter_mut().zip(&v.data) {
            *a = *a + *b;
        }
    }
}

// sub
impl<K> Vector<K>
where K: Sub<Output = K> + Copy {
    fn sub(&mut self, v: &Vector<K>) {
        for (a, b) in self.data.iter_mut().zip(&v.data) {
            *a = *a - *b;
        }
    }
}

//scl
impl<K> Vector<K>
where K: Mul<Output = K> + Copy {
    fn scl(&mut self, scalar: &K) {
        for a in self.data.iter_mut() {
            *a = *a * *scalar;
        }
    }
}


// From trait
impl<K, const N: usize> From<[K; N]> for Vector<K> where K: Copy {
    fn from(item: [K; N]) -> Self {
        Vector {data: item.to_vec()}
    }
}

// Display trait
impl<K: fmt::Display + std::fmt::Debug> fmt::Display for Vector<K> {
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

fn vector_tests() {
    println!("{GREEN}============= ADD ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u = Vector::from([2.,3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("{CYAN}{}{END}", u);
    // [7.0]
    // [10.0]

    println!("{GREEN}============= SUB ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    println!("{CYAN}{}{END}", u);
    // [-3.0]
    // [-4.0]

    println!("{GREEN}============= SCL ============={END}");

    println!("{YELLOW}Example tests{END}");
    let mut u = Vector::from([2., 3.]);
    u.scl(&2.);
    println!("{CYAN}{}{END}", u);
    // [4.0]
    // [6.0]

}

fn matrix_tests() {

}

fn main() {
    println!("\n\n{GREEN}   RUN VECTOR TESTS    {END}\n");
    vector_tests();

    println!("\n\n{GREEN}   RUN MATRIX TESTS    {END}\n");
    matrix_tests();
}
