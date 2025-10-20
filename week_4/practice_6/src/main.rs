use std::io;

fn main(){

    println!("Enter lower bound");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let lower_bound:i32 = input1.trim().parse().expect("Failed to input");

    println!("enter upper bound");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let upper_bound:i32 = input2.trim().parse().expect("failed to read input");

    for x in lower_bound..upper_bound{
        println!("Count level is {}", x);
    }


}