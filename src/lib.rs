pub mod common;
pub mod format;
pub mod parse;

pub fn format_file(filepath: &str) {
    use std::fs;
    let mut buffer: Vec<String> = vec![];
    {
        use std::io::{BufRead, BufReader};
        let file = fs::File::open(filepath)
            .unwrap_or_else(|e| panic!("Could not open file: {filepath}. {e}"));
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(x) => match parse::parse_line(&x) {
                    Ok(ll) => buffer.push(format::format_line(&ll)),
                    Err(e) => panic!("{e:#?}"),
                },
                Err(e) => panic!("{e}"),
            }
        }
    }
    {
        use std::io::Write;
        let mut file = fs::File::options()
            .write(true)
            .open(filepath)
            .unwrap_or_else(|e| panic!("Could not open file: {filepath}. {e}"));
        for line in &buffer {
            file.write_all(format!("{line}\n").as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
mod test {}
