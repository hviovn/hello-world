fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    for i in 1..=1_000_000 {
        if is_prime(i) {
            println!("{} is a prime number", i);
        }
    }
}