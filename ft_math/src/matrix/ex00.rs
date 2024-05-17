use crate::matrix::Matrix;

impl Matrix{
    pub fn add(&mut self, v: &Matrix){
        if self.shape() != v.shape(){
            println!("Matrixes are of different size");
            return ;
        }
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1{
                self.array[i][j] += v.array[i][j];
            }
        }
    }
    pub fn sub(&mut self, v: &Matrix){
        if self.shape() != v.shape(){
            println!("Matrixes are of different size");
            return ;
        }
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1{
                self.array[i][j] -= v.array[i][j];
            }
        }
    }
    pub fn scl(&mut self, a: f32){
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1{
                self.array[i][j] *= a;
            }
        }
    }
}