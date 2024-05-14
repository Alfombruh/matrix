use ft_math::vector::Vector;

fn main() {
    let mut juan = Vector::from(&[31., 32.]);
    {
        let this = &mut juan;
        let v = Vector::from(&[33., 33., 33.]);
        if v.size() != this.size() {
            //TO-DO throw exception
            return;
        }
    };
    // juan.push(33.);
    // juan.print();
    // println!("Lenght of the vector is {}", juan.size());
    // juan.print();
    // juan.remove_last();
    // juan.print();
    // juan.push(32.);
    // juan.push(32.);
    // juan.push(32.);
    // juan.push(32.);
    // juan.push(32.);
    // juan.print();
}
