fn test_divisibility_by_3_4(n: i32) -> i32 {
    if n % 3 == 0 && n % 4 == 0 {
        0
    } else if n % 3 == 0 {
        1
    } else if n % 4 == 0 {
        2
    } else {
        -1 // Optional: return -1 if divisible by neither
    }
}

fn main() {
    let number = 12; // Change this number to test other inputs
    let result = test_divisibility_by_3_4(number);

    match result {
        0 => println!("{} is divisible by both 3 and 4", number),
        1 => println!("{} is divisible by 3 only", number),
        2 => println!("{} is divisible by 4 only", number),
        _ => println!("{} is divisible by neither 3 nor 4", number),
    }
}
