use core::fmt::Debug;

/* STRUCTURE DEFINITION */
pub struct Vector<K> {
    pub  array: Vec<K>,
}

/* CONSTRUCTORS */
impl<K> Vector<K>{
    pub fn new(values: Vec<K>) -> Self{
        Self {array: values}
    }
}

/* METHOD IMPLEMENTATIONS */
impl<K: Debug> Vector<K> {
    pub fn add(&mut self, num: K) {
        self.array.push(num);
    }
    pub fn remove_last(&mut self) {
        self.array.remove(self.array.len() - 1);
    }
    pub fn size(&mut self) -> usize {
        return self.array.len();
    }
    pub fn print(&mut self) {
        print!("Vector is == (");
        for elem in &self.array{
            print!("{:?}, ", elem);
        }
        print!(");\n");
    }
}


