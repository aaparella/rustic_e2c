use std::env;
mod parser;    
use parser::Parser;

fn main() {
    let mut parser = Parser::new(env::args().nth(1).unwrap());
    parser.parse();
}
