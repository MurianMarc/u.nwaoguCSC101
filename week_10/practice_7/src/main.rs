struct Employee {
    name:String,
    company:String,
    age:u32
}

fn main(){
    let emp = vec![
        Employee{ name: "ebibiong jessica".to_string(), company: "enrst & young".to_string(), age : 25},
    ];
    println!("{:<20} {:<20} {:<6}", "name", "comapny", "age");
    println!("{:-<60}", " " );

    for employee in &emp{
        println!("{:<20} {:<20} {:<6}", employee.name, employee.company, employee.age);
    }

    
}