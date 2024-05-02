use core::fmt::Debug;
pub mod ex00;

/* STRUCTURE DEFINITION */
pub struct Vector<K> {
    array: Vec<K>,
}

/* CONSTRUCTORS */
impl<K> Vector<K>{
    pub fn from_vec(values: Vec<K>) -> Self{
        Self {array: values}
    }
    pub fn from(values: &[K]) -> Self where K: Copy
    {
        Self {array: values.to_vec()}
    }
}

/* METHOD IMPLEMENTATIONS */
impl<K: Debug> Vector<K> {
    pub fn push(&mut self, num: K) {
        self.array.push(num);
    }
    pub fn remove_last(&mut self) {
        self.array.remove(self.array.len() - 1);
    }
    pub fn size(&self) -> usize {
        return self.array.len();
    }
    pub fn print(&self) {
        println!("{:?}", &self.array);
    }
}


