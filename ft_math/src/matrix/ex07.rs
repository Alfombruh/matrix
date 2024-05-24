use crate::{matrix::Matrix, vector::Vector};

impl Matrix {
    pub fn mul_vec(&mut self, vec: Vector) -> Vector{
        return Vector::new(0)
    }
    pub fn mul_mat(&mut self, mat: Matrix) -> Matrix{
        return Matrix::from(&[&[0.]])

    }
}