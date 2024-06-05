use add_one;
use times_two;

fn main() {
    let num1 = 6;
    let num2 = 43;
    println!("Hello, world! {num1} + 1 is {}!", add_one::add_one(num1));
    println!("That's not all! {num2} * 2 is {}!", times_two::times_two(num2));
}
