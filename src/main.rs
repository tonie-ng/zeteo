mod well;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;
use std::process::exit;

struct Input {
    pattern: String,
    file_path: PathBuf,
}

fn main() {
    let args: Vec<String> = args().collect();
    let input = parse_input(args);

    let lines = read_file(input.file_path);
    find(lines, input.pattern);
}

fn find(lines: Lines<BufReader<File>>, pattern: String) {
    for line in lines {
        match line {
            Ok(line) => {
                if line.contains(&pattern) {
                    println!("{}", line);
                }
            }
            Err(_) => break,
        }
    }
}

fn read_file(file_path: PathBuf) -> Lines<BufReader<File>> {
    let f = File::open(file_path);
    match f {
        Ok(f) => {
            let reader = BufReader::new(f);
            reader.lines()
        }
        Err(err) => {
            println!("Error reading the file: {}", err.to_string());
            exit(1);
        }
    }
}

fn parse_input(args: Vec<String>) -> Input {
    match args.len() {
        3 => {
            let pattern = &args[1];
            let file_path = &args[2];

            let input = Input {
                pattern: String::from(pattern),
                file_path: PathBuf::from(file_path),
            };
            input
        }
        2 => {
            if args[1] == "--help" {
                println!("Usage ./zeteo <pattern> <file>\n");
                println!("pattern => string to look for\n");
                println!("file_path => path to the file to look through");
                exit(0);
            }
            println!("Usage: {:?} <pattern> <filepath>", &args[0]);
            exit(1);
        }
        _ => {
            println!("Usage: {:?} <pattern> <filepath>", &args[0]);
            exit(1);
        }
    }
}
