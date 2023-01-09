use std::env;
use std::io::{stderr, stdout, BufWriter, Write};

pub(crate) fn print_env() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current directory:");
    println!("{}", current_dir.display());
    println!();

    println!("Arguments:");
    let args: Vec<String> = env::args().collect();
    for it in args {
        println!("{}", it);
    }
    println!();

    println!("Environment variables:");
    let env_var = env::vars();
    for (key, value) in env_var {
        println!("{}={}", key, value);
    }

    Ok(())
}

pub(crate) fn print_message(print_to_stderr: bool, print_message_num: i32) -> std::io::Result<()> {
    if print_to_stderr {
        let out = stderr();
        let out = out.lock();
        let mut out = BufWriter::new(out);

        for _ in 0..print_message_num {
            out.write_all(b"The quick brown fox jumps over the lazy dog\r\n")?;
        }
        out.flush()?;
    } else {
        let out = stdout();
        let out = out.lock();
        let mut out = BufWriter::new(out);

        for _ in 0..print_message_num {
            out.write_all(b"The quick brown fox jumps over the lazy dog\r\n")?;
        }
        out.flush()?;
    }

    Ok(())
}
