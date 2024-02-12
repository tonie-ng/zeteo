use std::env::args;
use std::env::Args;
use std::path::PathBuf;
use std::process::exit;

struct Input {
    pattern: String,
    file_path: PathBuf,
}

fn main() {
    let args = args();
    let input = parse_input(args);

    println!("{:?}", input.file_path);
    println!("{:?}", input.pattern);
}

fn parse_input(mut args: Args) -> Input {
    match args.len() {
        3 => {
            let pattern: String = match args.nth(1) {
                Some(pat) => pat,
                None => {
                    println!("missing pattern");
                    exit(1);
                }
            };

            let file_path: String = match args.next() {
                Some(fp) => fp,
                None => {
                    println!("no file provided");
                    exit(1);
                }
            };
            let input = Input {
                pattern,
                file_path: PathBuf::from(file_path),
            };
            input
        }
        2 => {
            if let Some(arg) = args.nth(1) {
                if arg == "--help" {
                    println!("Usage ./zeteo <pattern> <file>");
                    println!("pattern => string to look for");
                    println!("file_path => path to the file to look through");
                    exit(0);
                }
            }
            println!("Usage: zeteo <pattern> <filepath>");
            exit(1);
        }
        _ => {
            println!("Usage: zeteo <pattern> <filepath>");
            exit(1);
        }
    }
}
