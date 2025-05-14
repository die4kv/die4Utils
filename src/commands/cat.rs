use clap::Args;
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Args, Debug)]
pub struct CatArgs {

    #[arg(short = 'n', long = "number")]
    pub number_lines: bool,

    pub files: Vec<String>,
}

pub fn run_args(args: CatArgs) {
    if args.files.is_empty() {
        eprintln!("{}", "cat: no files provided".red());
        return;
    }

    for file in args.files {
        match File::open(&file) {
            Ok(f) => {
                let reader = BufReader::new(f);
                for (i, line) in reader.lines().enumerate() {
                    match line {
                        Ok(text) => {
                            if args.number_lines {
                                println!(
                                    "{:>6}  {}",
                                    (i + 1).to_string().cyan(),
                                    text.bright_white()
                                );
                            } else {
                                println!("{}", text.bright_white());
                            }
                        }
                        Err(e) => {
                            eprintln!("{}: {}: {}", "cat error".red(), file, e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: {}: {}", "cat error".red(), file, e);
            }
        }
    }
}
