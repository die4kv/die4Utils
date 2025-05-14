use clap::Args;
use colored::*;

#[derive(Args, Debug)]
pub struct EchoArgs {

    #[arg(short = 'n', long = "no-newline")]
    pub no_newline: bool,
    pub text: Vec<String>,
}

pub fn run_args(args: EchoArgs) {
    let output = args.text.join(" ");

    if args.no_newline {
        print!("{}", output.bright_white());
    } else {
        println!("{}", output.bright_white());
    }
}
