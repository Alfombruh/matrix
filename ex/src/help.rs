use colored::Colorize;

fn main() {
    let message1 = "In order to test each exersice you need to call the Makefile this way:\n".bold().purple();
    print!("{}", message1);
    println!("make ex00/ex01/ex02/ex..");
}
