use crate::vector::Vector;

impl Vector 
{
    pub fn add(&mut self, v: &Vector) {
        if v.size() != self.size() {
            println!("Vectors are of different sizes");
            return;
        }
        for i in 0..self.size() {
            self.array[i] = self.array[i] + v.array[i];
        }
    }
    pub fn sub(&mut self, v: &Vector) {
        if self.size() != v.size() {
            println!("Vectors are of different sizes");
            return;
        }
        for i in 0..self.size() {
            self.array[i] = self.array[i] - v.array[i];
        }
    }
    pub fn scl(&mut self, a: f32) {
        for i in 0..self.size() {
            self.array[i] = a * self.array[i];
        }
    }
}
