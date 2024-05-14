use std::vec;

use ft_math::{matrix::Matrix, vector::Vector};
use colored::Colorize;

fn vec_test(){
    println!("{}", "///VECTOR TESTS///".blue().bold());
    let vect = Vector::from_vec(vec![11.567, -20.305]);
    print!("{}", "f64 vector -> ".red().bold());
    vect.print();
    let mut vect = Vector::from(&[2, 3]);
    print!("{}", "i32 vector -> ".red().bold());
    vect.print();
    println!("{}{}{}", "Size of vector is -> ".purple().bold() ,vect.size() ,"\nNow adding new elements to the vector".purple().bold());
    println!("{}", "Adding 20".purple().bold());
    vect.push(20.);
    println!("{}", "Adding -37".purple().bold());
    vect.push(-37.);
    println!("{}", "Adding 34".purple().bold());
    vect.push(34.);
    println!("{}", "Adding 299".purple().bold());
    vect.push(-299.);
    print!("{}", "i32 vector -> ".red().bold());
    vect.print();
    println!("{}{}{}", "New vector size is -> ".purple().bold(), vect.size() ,"\nNow removing 2 elements from it".purple().bold());
    vect.remove_last();
    vect.remove_last();
    print!("{}", "i32 vector -> ".red().bold());
    vect.print();
    println!("{}{}","Shortened vector size is -> ".purple().bold(), vect.size());
}

fn vec_errors(){
    let mut vect: Vector = Vector::from(&[133.3, -3.24, 9.77]);
    vect.remove_last();
    vect.remove_last();
    vect.remove_last();
    vect.remove_last();
}

// fn matrix_test(){
//     let matr = Matrix::from(&[&[1, 2]]);
//     matr.print();
//     let matr = Matrix::from(&[&[1, 2], &[2, 4]]);
//     matr.print();

// }

fn main(){
    vec_test();
    vec_errors();
}