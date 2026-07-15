mod input;
use reedline::Signal;
use std::env;
use std::process::Command;

fn tokenize(input: &str) -> Option<(Vec<&str>, &str)> {
    let mut parts = input.trim().split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return Option::None,
    };
    let args: Vec<&str> = parts.collect();
    Option::Some((args, cmd))
}

struct Execute;

impl Execute {
    fn run(cmd: &str, args: Vec<&str>) {
        match cmd {
            "cd" => Self::cd(args),
            "dc" => Self::dc(),
            "pwd" => Self::pwd(),
            "whtshl" => Self::whtshl(),
            "history" => Self::history(),
            _ => Self::external(cmd, args),
        }
    }

    fn cd(args: Vec<&str>) {
        let target = if args.is_empty() { 
            match env::var("HOME") {
                Ok(a) => &a.to_string(),
                Err(_) => "/", 
            }
        } else { args[0] };
        if let Err(e) = env::set_current_dir(target) {
            eprintln!("afsh: {}", e);
        }
    }

    fn dc() {
        let target = "..";
        if let Err(e) = env::set_current_dir(target) {
            eprintln!("afsh: {}", e);
        }
    }

    fn pwd() {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => println!("afsh: {}", e),
        }
    }

    fn whtshl() {
        println!("afsh");
    }

    fn history() {
        let hist_path = dirs::home_dir().unwrap_or(std::path::PathBuf::from("/"))
            .join(".afsh_history");
        match std::fs::read_to_string(&hist_path) {
            Ok(a) => {
                for (i,l) in a.lines().enumerate() {
                    println!("{:>4} {}", i+1, l);
                }
            },
            Err(e) => { println!("afsh {}", e); },
        }
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
    let mut rl = input::build_rl();
    let prompt = input::AfshPrompt;

    loop{
        let sig = rl.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                if &buffer == "q" { break; }
                let (args, cmd) = match tokenize(&buffer){
                    Option::Some((a, c)) => (a, c),
                    Option::None => continue,
                };
                Execute::run(cmd,args);
                if let Err(e) = rl.history_mut().sync() {
                    eprintln!("afsh: {}", e);
                };
            },
            Ok(Signal::CtrlC) => continue,
            Ok(Signal::CtrlD) => break,
            Ok(_) => todo!(),
            Err(e) => {
                println!("afsh: {}", e);
                break;
            }
        }
    }
}

