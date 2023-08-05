extern crate num;

use num::{bigint::BigUint, One};

fn factorial(n: u32) -> BigUint {
    (2..n).fold(BigUint::one(), |x, n| x * n)
}

fn main() {
    let result: BigUint = factorial(100);

    println!("Result: {}", result);

    let result_string = result.to_string();

    let sum: u32 = result_string
        .chars()
        .filter_map(|chr| chr.to_digit(10))
        .sum();

    println!("Sum: {:?}", sum);
}
