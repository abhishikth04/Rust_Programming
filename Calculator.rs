fn main() {
    let num1 = 10.0;
    let num2 = 5.0;
    let operator = '+'; // Change to '+', '-', '*', or '/'

    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Cannot divide by zero.");
                return;
            }
        }
        _ => {
            println!("Invalid operator.");
            return;
        }
    };

    println!("Result: {}", result);
}
