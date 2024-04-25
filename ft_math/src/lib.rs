pub struct Vector<K> {
    pub array: K,
}

impl<K> Vector<K> {
    pub fn changeValue(&mut self, num: K) {
        self.array = num;
    }
}

pub fn test() -> String {
    println!("juanmi");
    return String::from("juan marica");
}

pub struct Test<K> {
    pub data: K,
}

impl<K> Test<K> {
    pub fn change_value(&mut self, value: K){
        self.data = value;
    }
}