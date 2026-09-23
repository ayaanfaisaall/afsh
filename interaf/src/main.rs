use parsaf::{
    Lexer,
    Parser,
    Report,
};

fn main() {
    loop{
        let mut input = String::new();
        println!("Give Input Here: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed");

        let mut lexer = Lexer::new(&input);
        match lexer.tokenize() {
            Ok(a) => {
                let mut parser = Parser::new(&a);
                match parser.parse() {
                    Ok(ast) => {
                        println!("{:#?}", ast);
                    }
                    Err(e) => {
                        let report = Report::new(e).with_source_code(input.to_string());
                        println!("{:?}", report);
                    }
                }
            }
            Err(e) => {
                let report = Report::new(e).with_source_code(input.to_string());
                println!("{:?}", report);
            }
        }
    }
}
