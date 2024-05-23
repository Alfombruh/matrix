use crate::vector::Vector;

impl Vector{
    // Manhattan norm (https://medium.com/@hypotheticallytrue/what-is-a-manhattan-norm-of-a-vector-dd1561dbf335)
    pub fn norm_1(&mut self) -> f32{
        let mut result: f32 = 0.;
        for i in 0..self.size(){
             result += if self.array[i] < 0. {self.array[i] * (-1.)} else {self.array[i]};
        }
        return result;
    }
    // Euclidean norm (https://es.wikipedia.org/wiki/Norma_vectorial)
    pub fn norm(&mut self) -> f32{
        let mut result: f32 = 0.;
        for i in 0..self.size(){
             result += self.array[i].powf(2.);
        }
        return result.powf(0.5);
        
    }
    pub fn norm_inf(&mut self) -> f32{
        let mut result: f32 = 0.;
        for i in 0..self.size(){
            let abs = if self.array[i] > 0. {self.array[i]} else {-self.array[i]};
             result = if result >= abs {result} else {abs};
        }
        return result;
    }
}