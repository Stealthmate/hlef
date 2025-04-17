use std::fs;

pub mod common;
pub mod format;
pub mod parse;

pub fn load_config(filepath: Option<String>) -> common::Config {
    let Some(fp) = filepath else {
        return common::Config::default();
    };
    let Ok(data) = fs::read_to_string(fp) else {
        return common::Config::default();
    };
    toml::from_str(data.as_str()).expect("Bad config file.")
}

pub fn format_file(config: common::Config, filepath: &str) {
    use std::fs;
    let formatter = format::Formatter::new(config);
    let mut buffer: Vec<String> = vec![];
    {
        use std::io::{BufRead, BufReader};
        let file = fs::File::open(filepath)
            .unwrap_or_else(|e| panic!("Could not open file: {filepath}. {e}"));
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(x) => match parse::parse_line(&x) {
                    Ok(ll) => {
                        let formatted = formatter.format_line(&ll);
                        buffer.push(formatted);
                    }
                    Err(e) => panic!("{e:#?}"),
                },
                Err(e) => panic!("{e}"),
            }
        }
    }
    {
        use std::io::Write;
        let output_fp = format!("{}.hlef", filepath);
        let mut file = fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&output_fp)
            .unwrap_or_else(|e| panic!("Could not open file: {output_fp}. {e}"));
        for line in &buffer {
            file.write_all(format!("{line}\n").as_bytes()).unwrap();
        }

        fs::rename(output_fp, filepath).expect("Could not save file.");
    }
}

#[cfg(test)]
mod test {}
