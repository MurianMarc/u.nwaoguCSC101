fn main() {
    // An array of numbers
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);

    // Create a slice of 2nd and 3rd element (index 1 and 2)
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    // Omit the start index (starts from index 0 to 3 exclusive)
    let slice2 = &numbers[..3];
    println!("Index 0 to index 3 sliced = {:?}", slice2);

    // Omit the end index (starts from index 2 to end)
    let slice3 = &numbers[2..];
    println!("Index 2 to index 5 sliced = {:?}", slice3);

    // Omit both start and end index (entire array)
    let slice4 = &numbers[..];
    println!("Index 0 to index 5 sliced = {:?}", slice4);
}