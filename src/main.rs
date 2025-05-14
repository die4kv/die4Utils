mod commands;
use commands::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "die4Utils",
    version,
    about = "Kumpulan utilitas CLI seperti echo, cat, ls, find, dan grep",
    long_about = Some(
        "die4Utils adalah kumpulan tool baris perintah sederhana \
         yang ditulis dengan Rust. Setiap subcommand meniru fungsi \
         dari utilitas Unix populer."
    ),
    after_help = "Ketik 'die4Utils <command> --help' untuk bantuan tiap subcommand."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Echo {
        #[command(flatten)]
        args: echo::EchoArgs,
    },
    Cat {
        #[command(flatten)]
        args: cat::CatArgs,
    },
    Ls {
        #[command(flatten)]
        args: ls::LsArgs,
    },
    Find {
        #[command(flatten)]
        args: find::FindArgs,
    },
    Grep {
        #[command(flatten)]
        args: grep::GrepArgs,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Echo { args } => echo::run_args(args),
        Commands::Cat { args } => cat::run_args(args),
        Commands::Ls { args } => ls::run_args(args),
        Commands::Find { args } => find::run_args(args),
        Commands::Grep { args } => grep::run_args(args),
    }
}
