fn main() {
    let sum_of_squares: f64 = (1..=100).map(|x| (x as f64).powi(2)).sum();
    let square_of_sum: f64 = ((1..=100).sum::<u32>() as f64).powi(2);

    println!("Sum of squares: {}", sum_of_squares);
    println!("Square of sum: {}", square_of_sum);
    println!("Difference: {}", square_of_sum - sum_of_squares);
}
