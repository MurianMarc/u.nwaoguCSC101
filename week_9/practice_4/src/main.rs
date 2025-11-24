use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new().append(true).open("data.txt")?;

    let mut writer = BufWriter::new(file);

    writeln!(writer, "\nHello Class")?;
    writeln!(writer, "this is the appendage to the document.")?;

    println!("file append success");

    Ok(())


}