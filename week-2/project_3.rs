fn main (){
    //define the variables
    let p:f64 = 510_000.0;
    let r:f64 = 5.0;
    let t:f64 = 3.0;

    //calculate depriciation

    let d = p * (1.0 - (r/100.0)).powf(t);

    println! ("Your Depriciation after 3 years amounts to {}", d);
}