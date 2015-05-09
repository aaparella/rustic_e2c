use std::io;
use std::env;
use std::path;
use std::fs;
use std::io::BufStream;
use std::io::BufRead;

mod Scanner;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let mut file = match fs::File::open(path::Path::new(&filename)) {
        Ok(file) => file,
        Err(_) => panic!("Could not open file {}", filename),
    };
    let reader = BufStream::new(&file);
    let lines = reader.lines().map(|x| x.unwrap()).collect();
    let scanner = Scanner::new(&lines);
}
