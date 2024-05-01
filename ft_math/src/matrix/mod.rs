use core::fmt::Debug;

struct Matrix<T> {
    pub array: Vec<Vec<T>>
}

#[allow(dead_code)]
impl<T: Debug> Matrix<T> {
    /*
        m is the amout of rows
        n is the amount of colums
    */
    pub fn shape(&self)-> (usize, usize){
        return (self.array.len() ,self.array[0].len());
    }
    pub fn is_square(&self)-> bool{
        if self.shape().0 == self.shape().1 {
            return true;
        }
        return false;
    }
    pub fn print(&mut self){
        for vec in &self.array{
            println!("{:?}", vec)
        }
    }
}