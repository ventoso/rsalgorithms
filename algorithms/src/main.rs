use algorithms::fib::*;
use std::io;

fn main() {
    let mut inputs: String = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let num: i32 = inputs.trim().parse().unwrap();

    println!("{}", fib_recursion(num));
    println!("{}", fib_dp(num));
}
