use ft_math::{matrix::Matrix, vector::Vector};

fn vec_test(){
    let vect = Vector::from_vec(vec![2., 3.]);
    vect.print();
    let vect = Vector::from(&[2, 3]);

}

// fn matrxi_test(){
//     let matr = Matrix::from(&[&[1, 2]]);
//     matr.print();
//     let matr = Matrix::from(&[&[1, 2], &[2, 4]]);
//     matr.print();

// }

fn main(){
    vec_test();


}