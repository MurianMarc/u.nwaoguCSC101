use std::io;

fn main() {
    let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();

     println!("Enter the coefficient of X squared: ");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let a:f32 = input1.trim().parse().expect("not a valid number");

      println!("Enter the coefficient of X: ");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
     let b:f32 = input2.trim().parse().expect("not a valid number");

      println!("Enter the constant: ");
     io::stdin().read_line(&mut input3).expect("Not a valid string");
     let c:f32 = input3.trim().parse().expect("not a valid number");

    let d:f32 = b.powi(2) - (4.0 * a * c); //determinant
   
     if d < 0.0
    {
        println!("roots are imaginary");
    } else {

    let ro:f32 = (-b + d.sqrt()) / (2.0 * a) ; // root one

     let rt:f32 = (-b - d.sqrt()) / (2.0 * a) ; // root two

     println!(" the first root is {}", ro);

       println!(" the second root is {}", rt);
    }

    if d==0.0
     {
        println!("roots are equal");
    }
    if d > 0.0
    {
        println!("roots are unequal");
    }
   
}
 