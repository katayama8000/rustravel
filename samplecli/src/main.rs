use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "katayama8000",
    about = "Super awesome RPN calculator"
)]

struct Opts {
    // sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line)
        }
    } else {
        println!("no file specified");
    }
}
