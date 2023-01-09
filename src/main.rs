mod print;

use crate::print::print_env;
use clap::Parser;
use rand::random;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    /// Print current directory
    #[arg(long, short)]
    print_env: bool,

    #[arg(long)]
    crash_emulation: bool,

    #[arg(long, default_value = "0.5")]
    crash_rate: f32,

    #[arg(long, default_value = "-1")]
    exit_code: i32,
}

fn main() {
    let opt = Cli::parse();

    if opt.print_env {
        print_env().unwrap();
    }

    if opt.crash_emulation && opt.crash_rate > random() {
        exit(opt.exit_code);
    }
}
