fn pass_by_value(mut num: i32) {
    num += 1;
    println!("Inside pass_by_value: {}", num);
}

fn pass_by_reference(num: &mut i32) {
    *num += 1;
    println!("Inside pass_by_reference: {}", num);
}

fn main() {
    let val1 = 10;
    let mut val2 = 10;

    println!("Before pass_by_value: {}", val1);
    pass_by_value(val1); // val1 is not changed
    println!("After pass_by_value: {}", val1);

    println!("Before pass_by_reference: {}", val2);
    pass_by_reference(&mut val2); // val2 is changed
    println!("After pass_by_reference: {}", val2);
}
