fn main () {
    // defining the values
    let p:f64 = 520_000_000.0;
    let r:f64 = 10.0;
    let t:f64 = 5.0;

    

    //calculate the Amount and state it

    let a = p * (1.0 + (r/100.0)).powf(t);

    println!("For an Amount of {}", a);



    //calculate the Compound Interest and state it

    let ci = a - p;

     println!("your Compound Interest is {} naira", ci);

}