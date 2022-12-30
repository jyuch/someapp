use std::env;

fn main() -> std::io::Result<()> {
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
