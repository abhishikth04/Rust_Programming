fn main() {
    let mut count = 1;

    loop {
        if count > 10 {
            break;
        }
        println!("{}", count);
        count += 1;
    }
}
