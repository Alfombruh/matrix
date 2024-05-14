pub mod ex00;

/* STRUCTURE DEFINITION */
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
}

/* METHOD IMPLEMENTATIONS */
impl Vector {
    pub fn push(&mut self, num: f32) {
        self.array.push(num);
    }
    pub fn remove_last(&mut self) {
        if self.size() == 0 {
            //TO-DO Throw exeption
            return;
        }
        self.array.remove(self.array.len() - 1);
    }
    pub fn size(&self) -> usize {
        return self.array.len();
    }
    pub fn print(&self) {
        println!("{:?}", &self.array);
    }
}
