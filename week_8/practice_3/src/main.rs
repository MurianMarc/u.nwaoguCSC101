// Method to print the get value
fn value(n: Option<&char>) {
    println!("Element of vector: {:?}", n);
}

fn main() {
    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N'];
    let mut input1 = String::new();

    println!("\nEnter an index value between 0 and 8:");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    // Parse the input string to a usize
    let index: usize = input1.trim().parse().expect("Invalid input");

    // Get the value at the given index
    let ch: Option<&char> = v.get(index);
    value(ch);
}