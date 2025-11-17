use std::io;

fn main() {
    // Create an empty vector "city"
    let mut city: Vec<String> = Vec::new();

    // Print City Vector
    println!("The City vector has {} elements", city.len());

    // Ask how many cities to enter
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    let city_num: usize = input1.trim().parse().expect("Invalid input");

    // Loop to collect city names
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}:", count + 1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");

        let new_city = input2.trim().to_string();
        city.push(new_city);
    }

    // Display the entered cities
    println!("\nYour preferred cities are:");
    for (i, city_name) in city.iter().enumerate() {
        println!("{}: {}", i + 1, city_name);
    }
}