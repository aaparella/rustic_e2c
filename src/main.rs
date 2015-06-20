use std::env;
mod scanner;
use scanner::Scanner;

fn main() {
    scanner::test();
    let scanner = Scanner::new(env::args().nth(1).unwrap());
}
