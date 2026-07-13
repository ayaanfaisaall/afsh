use std::env;
use std::io::{self, Write};
use std::process::Command;

enum ShellState<T> {
    Kuch(T),
    Continue,
    Break,
}

fn input() -> ShellState<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => ShellState::Break,
        Ok(_) => ShellState::Kuch(input),
        Err(e) => {
            println!("error reading input {}", e);
            ShellState::Continue
        }
    }
}

fn tokenize(input: &str) -> ShellState<(Vec<&str>, &str)> {
    let mut parts = input.trim().split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return ShellState::Continue,
    };
    let args: Vec<&str> = parts.collect();
    ShellState::Kuch((args, cmd))
}

struct Execute;

impl Execute {
    fn run(cmd: &str, args: Vec<&str>) {
        match cmd {
            "q" => Self::q(),
            "dk" => Self::dk(args),
            "kd" => Self::kd(),
            "ldk" => Self::ldk(),
            "whtshl" => Self::whtshl(),
            _ => Self::external(cmd, args),
        }
    }

    fn q() {
        std::process::exit(0);
    }

    fn dk(args: Vec<&str>) {
        let target = if args.is_empty() { "/home/ayaan" } else { args[0] };
        if let Err(e) = env::set_current_dir(target) {
            eprintln!("afsh: {}", e);
        }
    }

    fn kd() {
        let target = "..";
        if let Err(e) = env::set_current_dir(target) {
            eprintln!("afsh: {}", e);
        }
    }

    fn ldk() {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => println!("afsh: {}", e),
        }
    }

    fn whtshl() {
        println!("afsh");
    }

    fn external(cmd: &str, args: Vec<&str>) {
        match Command::new(cmd).args(&args).status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("afsh: {}", status);
                }
            }
            Err(e) => {
                eprintln!("afsh: {}", e);
            }
        }
    }
}

fn main() {
    loop {
        let path = env::current_dir().unwrap();
        let path_str = path.to_string_lossy();
        let display_path = match env::var("HOME") {
            Ok(home) if path_str.starts_with(&home) => path_str.replacen(&home, "~", 1),
            _ => path_str.into_owned(),
        };
        
        print!("\x1b[1;38;2;50;130;224m❱❱{}\x1b[0m$ ", display_path);
        if let Err(e) = io::stdout().flush() {
            eprintln!("afsh: error: {}", e);
            break;
        }

        let input_str = match input() {
            ShellState::Kuch(i) => i,
            ShellState::Continue => continue,
            ShellState::Break => break,
        };

        let (args, cmd) = match tokenize(&input_str) {
            ShellState::Kuch((a, c)) => (a, c),
            ShellState::Continue => continue,
            ShellState::Break => unreachable!(),
        };

        Execute::run(cmd, args);
    }
}

