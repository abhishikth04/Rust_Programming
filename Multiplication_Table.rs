fn main() {
    let number = 5;
    let mut i = 1;

    'outer: loop {
        if i > 10 {
            break 'outer;
        }

        println!("{} x {} = {}", number, i, number * i);
        i += 1;
    }
}
