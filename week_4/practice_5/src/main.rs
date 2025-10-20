use std::io;

fn main(){
    let mut input = String::new();

    println!("\nEnter your height (in centimetres):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a Valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("you are of average height person");
    }
    else if height >= 170.0 && height <= 195.0
    {
        println!("you are tall");
    }
    else if height >= 100.0 && height <= 150.0
    {
        println!("you are short");
    }
    else
    {
        println!("Abnormal height");
    }
}
