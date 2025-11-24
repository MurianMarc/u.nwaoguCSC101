use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("table.txt")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Lager\t                 stout\t              Non-Alcoholic")?;
    writeln!(writer, "33 export\t            Legend\t            Maltine")?;
    writeln!(writer, "Desperados\t            Turbo_king\t      Amstel_malta")?;
    writeln!(writer, "Goldberg\t              Willams\t          Malta")?;
    writeln!(writer, "gulder\t                \t                 Fayrouz")?;
    writeln!(writer, "heineken")?;
    writeln!(writer, "star")?;

    writer.flush()?;
    Ok(())
}