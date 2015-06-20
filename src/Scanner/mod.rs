use std::fs::File;
use std::io::Read;

pub struct Scanner {
    contents : String,
    curr_ch  : Option<char>,
    position : usize,
    line     : u16,
    put_back : bool,
}

impl Scanner { 
    pub fn new(filename : String) -> Scanner {
        let mut file = match File::open(&filename) {
            Ok(f) => f,
            Err(e) => panic!("Could not open {} : {}", filename, e),
        };
        let mut cont = String::new();
        let _ = file.read_to_string(&mut cont);
        Scanner { contents : cont, 
                  curr_ch  : None, 
                  position : 0,
                  line     : 0, 
                  put_back : false}
    }

    fn next_char(&mut self) -> Option<char> {
        self.position += 1;
        self.contents.chars().nth(self.position - 1)
    }

    // For now just output the value scanned
    pub fn scan(&mut self) {
        match self.put_back {
            true => { self.put_back = false; },
            false => { self.curr_ch = self.next_char(); }
        }

        match self.curr_ch {
            None     => { panic!("[ERROR] Scan called after EOF"); },
            Some(ch) => {
                if ch.is_alphabetic() {
                    println!("{} is alphabetic", ch);
                } else if ch.is_numeric() {
                    println!("{} is numeric", ch);
                } else {
                    println!("{} is neither", ch);
                }
            }
        }
    }
}
