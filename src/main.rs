mod print;

use crate::print::print_env;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    /// Print current directory
    #[arg(long, short)]
    print_env: bool,
}

fn main() {
    let opt = Cli::parse();

    if opt.print_env {
        print_env().unwrap();
    }
}
