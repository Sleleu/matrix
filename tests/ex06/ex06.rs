use matrix::colors::*;
use matrix::vector::Vector;

fn cross_product_test() {

    println!("\n\n{GREEN} CROSS PRODUCT TESTS {END}\n");
    
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]

    println!("\n\n{GREEN} OTHER TESTS {END}\n");

    let u = Vector::from([0., 0., 0.]);
    let v = Vector::from([0., 0., 0.]);
    println!("{CYAN}Zero vectors{END}");
    println!("{}", Vector::cross_product(&u, &v));
    // [0.]
    // [0.]
    // [0.]

    let u = Vector::from([1., 42.]);
    let v = Vector::from([0., 42.]);
    println!("{CYAN}Not in R3 = undefined{END}");
    println!("{CYAN}G{GREEN}O{YELLOW} P{CYAN}A{GREEN}N{YELLOW}I{CYAN}I{GREEN}I{YELLOW}I{CYAN}C{END}");
    println!("{CYAN}   ||||{END}");
    println!("{CYAN}   ||||{END}");
    println!("{CYAN}   ||||{END}");
    println!("{CYAN}   ||||{END}");
    println!("{CYAN}   ||||{END}");
    println!("{CYAN}||||||||||{END}");
    println!("{CYAN} ||||||||{END}");
    println!("{CYAN}  ||||||{END}");
    println!("{CYAN}   ||||{END}");
    println!("{CYAN}    |||{END}");
    println!("{}", Vector::cross_product(&u, &v));
    // [0.]
    // [0.]
    // [0.]

}

fn main() {
    cross_product_test();
}