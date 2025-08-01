fn calculate_area_perimeter(x: u32, y: u32) -> (u32, u32) {
    let area = x * y;
    let perimeter = 2 * (x + y);
    (area, perimeter)
}

fn main() {
    let length = 5;
    let width = 3;
    
    let (area, perimeter) = calculate_area_perimeter(length, width);
    
    println!("For a rectangle with length {} and width {}:", length, width);
    println!("Area = {}", area);
    println!("Perimeter = {}", perimeter);
}
