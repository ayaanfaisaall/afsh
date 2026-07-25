mod input;
use reedline::Signal;
use std:: {
    env, process::Command, path::PathBuf,
    fs::File, io:: {BufRead, BufReader},
};

fn tokenize(input: &str) -> Option<(Vec<&str>, &str)> {
    let mut parts = input.trim().split_whitespace();
    let cmd = parts.next()?;
    let args: Vec<&str> = parts.collect();
    Some((args, cmd))
}

struct Shell;

impl Shell {
    fn run(cmd: &str, args: &[&str]) {
        match cmd {
            "cd" => Self::cd(args),
            "dc" => Self::dc(),
            "pwd" => Self::pwd(),
            "whtshl" => Self::whtshl(),
            "history" => Self::history(),
            _ => Self::external(cmd, args),
        }
    }

    fn cd(args: &[&str]) {
        let target = match args.first() {
            None | Some(&"~") => dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
            Some(&path) if path.starts_with("~/") => {
                let mut p = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
                p.push(&path[2..]);
                p
            },
            Some(&path) => PathBuf::from(path), 
        }; 

        if let Err(e) = env::set_current_dir(&target) {
            eprintln!("afsh: failed to change directory: {e}");
        }
    }

    fn dc() {
        let target = "..";
        if let Err(e) = env::set_current_dir(&target) {
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
        match File::open(&hist_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for (i, line) in reader.lines().enumerate() {
                    if let Ok(l) = line {
                        println!("{:>4}    {}", i+1, l);
                    }
                }
            }
            Err(e) => eprintln!("afsh: failed to load history: {}", e)
        }        
    }

    fn external(cmd: &str, args: &[&str]) {
        match Command::new(cmd).args(args).status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("afsh: {}", status);
                }
            },
            Err(e) => {
                eprintln!("afsh: {}", e);
            },
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
                let buffer = buffer.trim();
                if buffer == "q" || buffer == "exit" {
                    break;
                }
                if let Some((args, cmd)) = tokenize(buffer) {
                    Shell::run(cmd, &args);
                }
                if let Err(e) = rl.history_mut().sync() {
                    eprintln!("afsh: failed syncing history {}", e);
                };
            },
            Ok(Signal::CtrlC) => continue,
            Ok(Signal::CtrlD) => break,
            Ok(_) => {},
            Err(e) => {
                println!("afsh: {}", e);
                break;
            }
        }
    }
}

