use clap::Args;
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct FindArgs {

    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long = "name")]
    pub name: Option<String>,
}

pub fn run_args(args: FindArgs) {
    search_dir(&args.path, &args);
}

fn search_dir(path: &Path, args: &FindArgs) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                search_dir(&entry_path, args);
            }

            if let Some(name_pattern) = &args.name {
                if let Some(file_name) = entry_path.file_name().and_then(|n| n.to_str()) {
                    if file_name.contains(name_pattern) {
                        println!("{}", entry_path.display().to_string().green());
                    }
                }
            } else {
                println!("{}", entry_path.display().to_string().green());
            }
        }
    } else {
        eprintln!("{}: cannot access directory '{}'", "find error".red(), path.display());
    }
}
