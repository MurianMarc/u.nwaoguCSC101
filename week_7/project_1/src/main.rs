use std::io;


fn read_number(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}
fn area_trapezium() -> f32 {
    let h = read_number("Enter height:");
    let b1 = read_number("Enter base 1:");
    let b2 = read_number("Enter base 2:");
    (h / 2.0) * (b1 + b2)
}
fn area_rhombus() -> f32 {
    let d1 = read_number("Enter diagonal 1:");
    let d2 = read_number("Enter diagonal 2:");
    0.5 * d1 * d2
}
fn area_parallelogram() -> f32 {
    let base = read_number("Enter base:");
    let altitude = read_number("Enter altitude:");
    base * altitude
}
fn volume_cube() -> f32 {
    let side = read_number("Enter length of the cube side:");
    6.0 * side * side
}
fn volume_cylinder() -> f32 {
    let radius = read_number("Enter radius:");
    let height = read_number("Enter height:");
    std::f32::consts::PI * radius * radius * height //used standard value for pi
}
fn main() {
    println!("Select a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube (Surface Area)");
    println!("5. Volume of Cylinder");
    println!("Enter your choice (1-5):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice = choice.trim().parse::<u32>().unwrap_or(0);

    let result = match choice {
        1 => area_trapezium(),
        2 => area_rhombus(),
        3 => area_parallelogram(),
        4 => volume_cube(),
        5 => volume_cylinder(),
        _ => {
            println!("Invalid choice!");
            return;
        }
    };

    println!("Result = {}", result);
}