use core::fmt::Debug;

pub struct Vector<K> {
    pub array: Vec<K>,
}

impl<K: Debug> Vector<K> {
    pub fn add(&mut self, num: K) {
        self.array.push(num);
    }
    pub fn remove_last(&mut self) {
        self.array.remove(self.array.len() - 1);
    }
    pub fn len(&mut self) -> usize {
        return self.array.len();
    }
    pub fn print_vector(&mut self) {
        print!("Vector is == (");
        for elem in &self.array{
            print!("{:?}, ", elem);
        }
        print!(");\n");
    }
}

