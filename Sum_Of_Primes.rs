fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let limit = 10;
    let mut sum = 0;

    for num in 0..=limit {
        if is_prime(num) {
            sum += num;
        }
    }

    println!("Sum of prime numbers from 0 to {} is {}", limit, sum);
}
