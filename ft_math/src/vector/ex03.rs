use crate::vector::Vector;

impl Vector {
    pub fn dot(&self, v: Vector) -> f32{
        if self.size() != v.size() {
            println!("You cannot get the dot product of 2 different sized vectors");
            return 0.;
        }
        let mut sum: f32 = 0.;
        for i in 0..v.size(){
            sum += self.array[i] * v.array[i];
        }
        return sum;
    }
}