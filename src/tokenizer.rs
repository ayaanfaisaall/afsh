use std::str::Chars;
use std::iter::Peekable;
#[path = "tokenizer/tokens.rs"]
mod tokens;
use tokens::Token;

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
   fn new(input: &'a str) -> Self {
       Lexer {
           chars: input.chars().peekable(),
       }
   } 

   fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\n' | '\t' | '\r' => {
                    self.chars.next();
                },
                '0'..'9' => {
                    let mut num_str = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        if ch.is_digit(10) {
                            num_str.push(ch);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Num(&num_str));
                },
                _ => todo!(),
            }
        }
        tokens
   }
}

fn main() {
    let code = " mera naam 99 8 55 7777 \n ";
    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();
    println!("{:?}",tokens);
}
