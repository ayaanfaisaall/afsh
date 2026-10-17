mod input;
use parsaf::{
    Report,
    Parser,
    Lexer
};
use reedline::Signal;

fn main() {
    let mut rl = input::build_rl();
    let prompt = input::AfshPrompt;
    ctrlc::set_handler(move || {}).expect("error: setting ctrlc");

    loop{
        let sig = rl.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let buffer = buffer.trim();
                if buffer == "q" || buffer == "exit" {
                    break;
                }
                let mut lexer = Lexer::new(buffer);
                match lexer.tokenize() {
                    Ok(tokens) => {
                        let mut parser = Parser::new(&tokens);
                        match parser.parse() {
                            Ok(ast) => println!("{:#?}", ast),
                            Err(e) => {
                                let error = Report::new(e).with_source_code(buffer.to_string());
                                println!("{:?}", error);
                            }
                        }
                    }
                    Err(e) => {
                        let error = Report::new(e).with_source_code(buffer.to_string());
                        println!("{:?}", error);
                    }
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

