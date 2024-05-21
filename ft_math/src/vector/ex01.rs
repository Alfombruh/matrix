use crate::vector::Vector;

pub fn linear_combination(u: &[Vector], coefs: &[f32]) -> Vector
{
    let mut new_vec = Vector::new(u[0].size());
    if u.is_empty() || coefs.is_empty() {
        println!("both the vectors and the coeficients cannot be empty");
        return new_vec;
    }
    if u.len() != coefs.len() {
        println!("The number of vectors and coeficients has to be the same");
        return new_vec;
    }
    for (vector, & coef) in u.iter().zip(coefs){
        let mut scaled: Vector = vector.clone();
        scaled.scl(coef);
        new_vec.add(&scaled);
    }
    return new_vec;
} 