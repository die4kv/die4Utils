use clap::Args;
use colored::*;
use std::fs;
use std::path::Path;

#[derive(Args, Debug)]
pub struct LsArgs {

    #[arg(short = 'a', long = "all")]
    pub show_all: bool,

    #[arg(default_value = ".")]
    pub path: String,
}

pub fn run_args(args: LsArgs) {
    let path = Path::new(&args.path);

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if !args.show_all && file_name_str.starts_with('.') {
                    continue;
                }

                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                let display_name = if metadata.is_dir() {
                    file_name_str.green().bold()
                } else {
                    file_name_str.blue()
                };

                println!("{}", display_name);
            }
        }
        Err(e) => {
            eprintln!("{} '{}': {}", "ls error".red(), args.path, e);
        }
    }
}
