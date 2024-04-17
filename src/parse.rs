use std::path::PathBuf;
use std::process::exit;

pub struct Input {
    pub pattern: String,
    pub file_path: PathBuf,
    pub flag: String,
}

fn allowed_flags() -> Vec<&'static str> {
    vec!["-n", "-rev", "-revn"]
}

pub fn parse_flag(flag: &String) -> &str {
    let flag = flag.as_str();

    if !allowed_flags().contains(&flag) {
        println!("Unsupported flag {}", flag);
        exit(1);
    }
    flag
}

pub fn parse_input(args: Vec<String>) -> Input {
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
                println!("     -n: print a count of the number of lines with the occurence of the pattern");
                println!("     -rev: print all the lines without the occurence of the pattern ");
                println!("     -revn: print a count of the number of lines without the occurence of the pattern");
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
