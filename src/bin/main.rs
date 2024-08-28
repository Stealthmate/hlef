use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(num_args = 1..)]
    pub files: Vec<String>,
}

fn do_format_line(line: &str) -> String {
    if !line.starts_with("  ") {
        return line.to_owned();
    }

    let mut formatted = line.to_owned();
    formatted = formatted.trim().to_owned();

    match formatted.chars().next() {
        None => return formatted,
        Some(';') => return line.to_owned(),
        _ => {}
    };

    let re = regex::Regex::new(r"^   *([^ ]+)( +((\= )?[^ ]+)? +([^ ]+).*)?$").unwrap();
    let Some(results) = re.captures(line) else {
        panic!("Could not parse posting: {line}")
    };

    formatted = "  ".to_owned();

    if results.get(2).is_none() {
        formatted += results.get(1).unwrap().as_str();
    } else {
        formatted += &format!("{: <70}", results.get(1).unwrap().as_str());
        formatted += &format!("{: >5}", results.get(3).map(|x| x.as_str()).unwrap_or(""));
        formatted += &format!("{: >10}", results.get(5).map(|x| x.as_str()).unwrap_or(""));
    }

    formatted
}

fn do_format(filepath: &str) {
    use std::fs;
    let mut buffer: Vec<String> = vec![];
    {
        use std::io::{BufRead, BufReader};
        let file = fs::File::open(filepath)
            .unwrap_or_else(|e| panic!("Could not open file: {filepath}. {e}"));
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(x) => buffer.push(do_format_line(&x)),
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

fn main() {
    let args = Args::parse();

    for file in &args.files {
        do_format(file)
    }
}
