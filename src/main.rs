use std::env;
mod parser;    
use parser::Parser;
use parser::scanner::Scanner;

fn main() {
    let mut scanner = Scanner::new(env::args().nth(1).unwrap());
    let mut parser = Parser::new(&mut scanner);
    parser.parse();
}
