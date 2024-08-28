use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(num_args = 1..)]
    pub files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for file in &args.files {
        hlef::format_file(file)
    }
}
