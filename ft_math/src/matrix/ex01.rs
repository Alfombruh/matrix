use crate::matrix::Matrix;


//Doesnt work like that, but I'll leave it here
pub fn linear_combination(u: &[Matrix], coefs: &[f32]) -> Matrix
{
    let mut new_vec: Matrix = Matrix::new(u[0].shape().0, u[0].shape().1);
    if u.is_empty() || coefs.is_empty() {
        println!("both the vectors and the coeficients cannot be empty");
        return new_vec;
    }
    if u.len() != coefs.len() {
        println!("The number of vectors and coeficients has to be the same");
        return new_vec;
    }
    for (matrix, & coef) in u.iter().zip(coefs){
        let mut scaled: Matrix = matrix.clone();
        scaled.scl(coef);
        new_vec.add(&scaled);
    }
    return new_vec;
}   
