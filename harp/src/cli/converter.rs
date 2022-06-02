use std::io::{BufRead, Result};
use std::{fs::File, fs::OpenOptions, io::BufReader, path::PathBuf};

/// # convert your er-diagram to ddl
///
pub fn converte_to_ddl(input_path: PathBuf, output_path: PathBuf) -> Result<()> {
    let input_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&input_path)?;

    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        print!("{}", line.unwrap())
    }

    File::create(output_path).unwrap();

    Ok(())
}
