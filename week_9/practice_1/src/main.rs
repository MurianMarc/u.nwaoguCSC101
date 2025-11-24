use std::fs::File;
use std::io::{BufWriter, Write}; //it reduces syscalls when writing

fn main() -> std::io::Result<()> {
let announce = "Week 9 - rust file input & output";
let dept = "Department of computer science";
let extra = "i used bufwriter to shorten code and improve system perfomance";

let file = File::create("data.txt")?;
let mut writer = BufWriter::new(file);

writeln!(writer, "welcome to rust programming")?;
writeln!(writer, "{}", announce)?;
writeln!(writer, "{}", dept)?;
writeln!(writer, "{}", extra)?;

println!("\nData written to file.");

Ok(())

}