use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::path::Path;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a file name as an argument.");
        process::exit(1);
    }

    let filename = &args[1];
    let path = Path::new(filename);

    if !path.exists() {
        eprintln!("File does not exist: {}", path.display());
        process::exit(1);
    }
    
    println!("Arguments: {:?}", args);

    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("File not found: {}", error)
                }
                _ => {
                    eprintln!("Error opening file: {}", error)
                }
            }
            process::exit(1);
        }
    };
    
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                println!("Line {}: {}", index + 1, line);
            }
            Err(error) => {
                eprintln!("Error reading line {}: {}", index + 1, error);
            }
        }
    }
}