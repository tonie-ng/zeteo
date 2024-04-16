use crate::parse::parse_input;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;
use std::process::exit;

mod parse;
mod utils;

fn main() {
    let args: Vec<String> = args().collect();

    let input = parse_input(args);
    let lines = read_file(input.file_path);

    let flag = input.flag.as_str();
    match flag {
        "-n" => count(lines, input.pattern),
        "--rev" => find(lines, input.pattern, true),
        _ => find(lines, input.pattern, false),
    }
}

fn find(lines: Lines<BufReader<File>>, pattern: String, mode: bool) {
    for line in lines {
        match line {
            Ok(line) => print_line(mode, line, &pattern),
            Err(_) => break,
        }
    }
}

fn print_line(mode: bool, line: String, pattern: &String) {
    match mode {
        true => {
            if line.contains(*&pattern) {
                println!("{}", line)
            }
        }
        false => {
            if !line.contains(*&pattern) {
                println!("{}", line)
            }
        }
    }
}

fn count(lines: Lines<BufReader<File>>, pattern: String) {
    let mut line_count = 0;

    for line in lines {
        match line {
            Ok(line) => {
                if line.contains(&pattern) {
                    line_count += 1;
                }
            }
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
