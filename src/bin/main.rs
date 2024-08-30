use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(num_args = 1..)]
    pub files: Vec<String>,
}

fn main() {
    use rayon::prelude::*;

    let args = Args::parse();

    args.files.par_iter().for_each(|x| hlef::format_file(x));
}
