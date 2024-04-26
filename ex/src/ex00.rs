use ft_math;

fn main() {
    let mut juan = ft_math::Vector{ array: vec![32]};
    juan.add(33);
    println!("{}", juan.array[0]);
    println!("{}", juan.array[1]);
    println!("Lenght of the vector is {}", juan.len());
    println!("{:?}", juan.array);
    juan.remove_last();
    println!("{:?}", juan.array);
    juan.add(32);
    juan.add(32);
    juan.add(32);
    juan.add(32);
    juan.add(32);
    juan.print_vector();
    // println!("{}", juan.array[1]); //esta linea rompe (esta bien)
}
