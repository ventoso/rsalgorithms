pub mod fib {
    pub fn fib_recursion(n: i32) -> i32 {
        match n {
            0 => 0,
            1..=2 => 1,
            _ => fib_recursion(n - 1) + fib_recursion(n - 2),
        }
    }

    pub fn fib_dp(n: i32) -> i32 {
        let mut vec: Vec<i32> = Vec::new();

        vec.push(1);
        vec.push(1);
        for i in 2..=n {
            vec.push(vec[(i - 1) as usize] + vec[(i - 2) as usize]);
        }

        vec[(n - 1) as usize]
    }
}
