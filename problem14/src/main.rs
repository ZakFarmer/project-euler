const LOWER_BOUND: usize = 1;
const UPPER_BOUND: usize = 1000000;

fn main() {
    // println!("Collatz (13): {:?}", collatz(13));
    let mut largest_length = 0;
    let mut largest_x = 0;

    for x in LOWER_BOUND..UPPER_BOUND {
        let mut last: u64 = x as u64;
        let mut length = 0;

        while last != 1 {
            last = collatz(last);

            length += 1;
        }

        if length > largest_length {
            largest_length = length;
            largest_x = x;
        }

        println!("Length using x={}: {}", x, length);
    }

    println!("Largest length using x={} with {} numbers", largest_x, largest_length);
}

fn collatz(n: u64) -> u64 {
    let next: u64 = if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    };

    next
}