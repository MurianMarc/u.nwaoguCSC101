// declare a structure
struct Employee {
    ceo: String,
    company: String,
    age: u32,
}

fn main() {
    // create two structures
    let emp1 = Employee {
        ceo: String::from("Satya Nadella"),
        company: String::from("Microsoft Corporation"),
        age: 56,
    };
    let emp2 = Employee {
        ceo: String::from("Sundar Pichai"),
        company: String::from("Google Inc."),
        age: 51,
    };
    // pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}

// display employee details
fn display(emp: Employee) {
    println!("CEO is: {}, Age is: {}", emp.ceo, emp.age);
}