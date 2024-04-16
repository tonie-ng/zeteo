use crate::parse::parse_input;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;
use std::process::exit;

mod parse;
mod utils;

enum SearchMode {
    Reverse,
    Normal,
}

fn main() {
    let args: Vec<String> = args().collect();

    let input = parse_input(args);
    let lines = read_file(input.file_path);

    let flag = input.flag.as_str();
    match flag {
        "-n" => count(lines, input.pattern, SearchMode::Normal),
        "-revn" => count(lines, input.pattern, SearchMode::Reverse),
        "-rev" => find(lines, input.pattern, SearchMode::Reverse),
        _ => find(lines, input.pattern, SearchMode::Normal),
    }
}

fn find(lines: Lines<BufReader<File>>, pattern: String, mode: SearchMode) {
    for line in lines {
        match line {
            Ok(line) => match mode {
                SearchMode::Normal => {
                    if line.contains(&pattern) {
                        println!("{}", line)
                    }
                }
                SearchMode::Reverse => {
                    if !line.contains(&pattern) {
                        println!("{}", line)
                    }
                }
            },
            Err(_) => break,
        }
    }
}

fn count(lines: Lines<BufReader<File>>, pattern: String, mode: SearchMode) {
    let mut line_count = 0;

    for line in lines {
        match line {
            Ok(line) => match mode {
                SearchMode::Normal => {
                    if line.contains(&pattern) {
                        line_count += 1;
                    }
                }
                SearchMode::Reverse => {
                    if !line.contains(&pattern) {
                        line_count += 1;
                    }
                }
            },
            Err(_) => break,
        }
    }

    println!("line_count: {}", line_count)
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
