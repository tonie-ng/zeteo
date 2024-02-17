use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;
use std::process::exit;

struct Input {
    pattern: String,
    file_path: PathBuf,
    flag: String,
}

fn main() {
    let args: Vec<String> = args().collect();

    let input = parse_input(args);
    let lines = read_file(input.file_path);

    let flag = input.flag.as_str();
    match flag {
        "-n" => count(lines, input.pattern),
        _ => find(lines, input.pattern),
    }
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

fn allowed_flags() -> Vec<&'static str> {
    vec!["-n"]
}

fn parse_flag(flag: &String) -> &str {
    let flag = flag.as_str();

    if !allowed_flags().contains(&flag) {
        println!("Unsupported flag {}", flag);
        exit(1);
    }
    flag
}

fn parse_input(args: Vec<String>) -> Input {
    match args.len() {
        4 => {
            let pattern = &args[1];
            let file_path = &args[2];
            let flag = parse_flag(&args[3]);

            let input = Input {
                pattern: String::from(pattern),
                file_path: PathBuf::from(file_path),
                flag: String::from(flag),
            };
            input
        }
        3 => {
            let pattern = &args[1];
            let file_path = &args[2];

            let input = Input {
                pattern: String::from(pattern),
                file_path: PathBuf::from(file_path),
                flag: String::from("none"),
            };
            input
        }
        2 => {
            if args[1] == "--help" {
                println!("Usage: ./zeteo [PATTERN] [FILE_PATH]\n");

                println!("Flags:");
                println!("     -n: to count the number of lines");
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
