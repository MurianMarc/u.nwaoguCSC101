fn main() {
    let x = vec![100,200,300];
    borrow_vector(&x);
    println!("printing the vales from main() x[0]={}", x[0]);
    println!("{:*<40}", "");
}
fn borrow_vector(z:&Vec<i32>){
   println!("{:*<40}", "");
   println!("inside print_vector function {:?} \n ", z);
   println!("{:-<40}", "");
}
