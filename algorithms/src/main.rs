use std::io;

fn main() {
    println!("Please input some integer");

    let mut inputs: String = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let num: i32 = inputs.trim().parse().unwrap();

    println!("{}", fib(num));
}

fn fib(n: i32) -> i32 {
    // match statement
    match n {
        0 => 0,
        1..=2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }

    // traditional branch statement
    // if n == 0 {
    //     return 0;
    // } else if n == 1 || n == 2 {
    //     return 1;
    // } else {
    //     return fib(n - 1) + fib(n - 2);
    // }
}
