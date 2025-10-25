use std::io;

fn main(){
    let mut input = String::new();
    let mut exp = String::new(); //experience

    println!("do you have experience? (y/n)"); //experience question
    io::stdin().read_line(&mut exp).expect("Wrong input");

    let exp = exp.trim().to_lowercase(); 

    if exp == "y" {

    

    println!("\nEnter your Age in whole numbers:");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let age:u32 = input.trim().parse().expect("Not a Valid number");

    if age >= 30 && age <= 39
    {
        println!("your incentive is N1,480,000");
    }
    else if age > 0 && age <= 28
    {
        println!("your incentive is 1,300,000");
    }
    else if age >= 40 
    {
        println!("your incentive is N1,560,000");
    }
    else
    {
        println!("Abnormal age");
    }
} else if exp == "n" {
    println!("your incentive should be N100,000");
}
 else {
    println!(" enter a valid response");
}
}
