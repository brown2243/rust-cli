use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("rcp <target path> <destination path>");
        std::process::exit(1);
    }
    let source = &args[1];
    let destination = &args[2];
    println!("args:{} {}", source, destination);

    if !Path::new(source).exists() {
        println!("target is not exist: {}", source);
        std::process::exit(1);
    }

    match copy_file(source, destination) {
        Ok(_) => println!("copy success."),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn copy_file(source: &str, destination: &str) -> io::Result<()> {
    fs::copy(source, destination)?;
    Ok(())
}
