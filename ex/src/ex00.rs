use ft_math::{matrix::Matrix, vector::Vector};

fn main() {
    // let mut juan = Vector::from(&[31., 32.]);
    // {
    //     let this = &mut juan;
    //     let v = Vector::from(&[33., 33., 33.]);
    //     if v.size() != this.size() {
    //         //TO-DO throw exception
    //         return;
    //     }
    // };
    let mut matt: Matrix = Matrix::from(&[&[1.2, 3.4], &[4.4, 5.7]]);
    let matt2: Matrix = Matrix::from(&[&[6.2, -3.4], &[5.7, 2.2]]);
    matt.add(&matt2);
    matt.print();
    // juan.push(33.);
    // juan.print();
    // println!("Lenght of the vector is {}", juan.size());
    // juan.print();
    // juan.remove_last();
    // juan.print();
    // juan.push(32.);
    // juan.push(32.);
    // juan.push(32.);
    // juan.push(32.);
    // juan.push(32.);
    // juan.print();
}
