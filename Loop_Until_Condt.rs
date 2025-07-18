fn main() {
    let mut count = 0;

    loop {
        count += 1;

        if count == 7 {
            println!("Condition met at iteration {}", count);
            break;
        }
    }

    println!("Total iterations: {}", count);
}
