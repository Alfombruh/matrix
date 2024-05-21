use crate::matrix::Matrix;
pub mod ex00;
pub mod ex01;
pub mod ex03;

/* STRUCTURE DEFINITION */
#[derive(Debug)]
pub struct Vector {
    array: Vec<f32>,
}

/* CONSTRUCTORS */
impl Vector{
    pub fn from_vec(values: Vec<f32>) -> Self {
        Self { array: values }
    }
    pub fn from(vec: &[f32]) -> Self{
        Self { array: vec.to_vec() }
    }
    pub fn new(size: usize) -> Self{
        Self { array: vec![0.0; size]}
    }
}

/* METHOD IMPLEMENTATIONS */
impl Vector {
    pub fn push(&mut self, num: f32) {
        self.array.push(num);
    }
    pub fn remove_last(&mut self) {
        if self.size() == 0 {
            println!("Vectors are of different sizes");
            return;
        }
        self.array.remove(self.array.len() - 1);
    }
    pub fn size(&self) -> usize {
        return self.array.len();
    }
    pub fn to_matrix(&self) -> Matrix{
        return Matrix::from(&[self.array.as_slice()]);
    }
    pub fn print(&self) {
        println!("{:?}", &self.array);
    }
    pub fn clone(&self) -> Vector{
        return Vector{array: self.array.clone()};
    }
}

//MAKES THE CLASS PRINTEABLE WITH JUST {}
impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.array)
    }
}
