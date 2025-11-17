fn main() {
    // Define a tuple
    let b: (i32, bool, f64) = (110, true, 10.9);

    // Pass the tuple to the print function
    print_tuple(b);
}

// Function to print the tuple
fn print_tuple(x: (i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}