use ft_math::vector::Vector;

fn main() {
    let mut juan = Vector::new(vec![31., 21.]);
    juan.add(33.);
    println!("{}", juan.array[0]);
    println!("{}", juan.array[1]);
    println!("Lenght of the vector is {}", juan.size());
    println!("{:?}", juan.array);
    juan.remove_last();
    println!("{:?}", juan.array);
    juan.add(32.);
    juan.add(32.);
    juan.add(32.);
    juan.add(32.);
    juan.add(32.);
    juan.print();
    println!("{}", juan.array[1]); //esta linea rompe (esta bien)

}
