use std::io;

fn main() {
    println!("Welcome to the Food Ordering System!, here is your menu"); //disply menu
    
    println!("Poundo Yam/Edinkaiko Soup (p) - N3,200");
    println!("Fried Rice & Chicken (f)      - N3,000");
    println!("Amala & Ewedu Soup(a)        - N2,500");
    println!("Eba & Egusi Soup   (e)        - N2,000");
    println!("White Rice & Stew (w)         - N2,500");

    println!("\nPlease enter the letter of the item you want to order: (use the initials)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let choice = input.trim().to_uppercase(); //convert to uppercase in case lowercase is usd

    match choice.as_str() { //displaylowercase
        "P" => println!("You ordered Poundo Yam/Edinkaiko Soup. Price: N3,200"),
        "F" => println!("You ordered Fried Rice & Chicken. Price: N3,000"),
        "A" => println!("You ordered Amala & Ewedu Soup. Price: N2,500"),
        "E" => println!("You ordered Eba & Egusi Soup. Price: N2,000"),
        "W" => println!("You ordered White Rice & Stew. Price: N2,500"),
        &_ => println!("please use a valid input"),
        
    }
}