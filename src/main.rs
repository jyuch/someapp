mod print;

use crate::print::{print_env, print_message};
use clap::Parser;
use rand::random;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    /// Print current environment
    #[arg(long, short)]
    print_env: bool,

    /// Print message num
    #[arg(long, default_value = "0")]
    print_message_num: i32,

    /// Switch output stdout to stderr
    #[arg(long)]
    print_to_stderr: bool,

    /// Enable crash emulation
    #[arg(long)]
    crash_emulation: bool,

    /// Crash rate
    #[arg(long, default_value = "0.5")]
    crash_rate: f32,

    /// Exit code on crash
    #[arg(long, default_value = "-1")]
    exit_code: i32,
}

fn main() {
    let opt = Cli::parse();

    if opt.print_env {
        print_env().unwrap();
    }

    print_message(opt.print_to_stderr, opt.print_message_num).unwrap();

    if opt.crash_emulation && opt.crash_rate > random() {
        exit(opt.exit_code);
    }
}
