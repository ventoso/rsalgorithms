// 阶乘
// n! n*(n-1)*(n-2)....

fn factorial(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

fn main() {
    println!("{}", factorial(5));
}
