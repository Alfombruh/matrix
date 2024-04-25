use ft_math;

fn main() {
    println!("{}", ft_math::test());
    let mut juan = ft_math::Vector {array: 32};
    println!("{}", juan.array);
    juan.changeValue(4432);
    println!("{}", juan.array);
}
