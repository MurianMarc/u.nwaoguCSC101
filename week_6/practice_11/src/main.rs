
fn main() {
    let a: i32 = 2; // Bit representation: 10
    let b: i32 = 3; // Bit representation: 11

    let mut result: i32;

    result = a & b;
    println!("(a & b) -> {}", result); //  AND

    result = a | b;
    println!("(a | b) -> {}", result); //  OR

    result = a ^ b;
    println!("(a ^ b) -> {}", result); // XOR

    result = !b;
    println!("(!b) -> {}", result); 

    result = a << b;
    println!("(a << b) -> {}", result); 

    result = a >> b;
    println!("(a >> b) -> {}", result); 
}




