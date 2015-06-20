use std::env;
mod scanner;
use scanner::Scanner;

fn main() {
    let mut scanner = Scanner::new(env::args().nth(1).unwrap());
}
