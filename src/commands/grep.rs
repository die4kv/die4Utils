use clap::Args;
use colored::*;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Args, Debug)]
pub struct GrepArgs {
    pub pattern: String,
    #[arg(default_value = ".")]
    pub path: PathBuf,
    #[arg(short, long)]
    pub ignore_case: bool,
    #[arg(short, long)]
    pub recursive: bool,
}

pub fn run_args(config: GrepArgs) {
    if config.recursive {
        search_dir(&config.path, &config);
    } else if config.path.is_file() {
        search_file(&config.path, &config);
    } else {
        eprintln!("{} is not a file. Use -r to search directories.", config.path.display());
    }
}

fn search_dir(path: &Path, config: &GrepArgs) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                search_dir(&path, config);
            } else if path.is_file() {
                search_file(&path, config);
            }
        }
    } else {
        eprintln!("{}: failed to read directory", path.display());
    }
}

fn search_file(path: &Path, config: &GrepArgs) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", path.display(), e);
            return;
        }
    };

    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        if let Ok(text) = line {
            let matched = if config.ignore_case {
                text.to_lowercase().contains(&config.pattern.to_lowercase())
            } else {
                text.contains(&config.pattern)
            };

            if matched {
                let highlighted = if config.ignore_case {
                    highlight_ignore_case(&text, &config.pattern)
                } else {
                    text.replace(&config.pattern, &config.pattern.red().bold().to_string())
                };

                println!(
                    "{}:{}: {}",
                    path.display().to_string().blue(),
                    (i + 1).to_string().cyan(),
                    highlighted
                );
            }
        }
    }
}

fn highlight_ignore_case(text: &str, pattern: &str) -> String {
    let mut result = String::new();
    let lower_text = text.to_lowercase();
    let lower_pattern = pattern.to_lowercase();

    let mut last = 0;
    let mut offset = 0;

    while let Some(pos) = lower_text[offset..].find(&lower_pattern) {
        let real_pos = offset + pos;
        result.push_str(&text[last..real_pos]);
        result.push_str(&text[real_pos..real_pos + pattern.len()].red().bold().to_string());
        offset = real_pos + pattern.len();
        last = offset;
    }

    result.push_str(&text[last..]);
    result
}
