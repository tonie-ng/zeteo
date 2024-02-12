use std::env::args;
use std::path::PathBuf;
use std::process::exit;

struct Input {
    pattern: String,
    file_path: PathBuf,
}

fn main() {
    let args: Vec<String> = args().collect();
    let input = parse_input(args);

    println!("{:?}", input.file_path);
    println!("{:?}", input.pattern);
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
