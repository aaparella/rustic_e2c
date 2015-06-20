use std::fs::File;
use std::io::Read;

struct Token {
    typ  : TokenType,
    line : u16,
}

enum TokenType {
    VAR,   RAV,   PRINT,
    IF,    FI,    DO,
    OD,    ELSE,  FA,
    AF,    TO,    ST,

    ASSIGN,    LPAREN,
    RPAREN,    PLUS,
    MINUS,     TIMES,    DIVIDE,
    EQ,  NE,  LT,
    GT,  LE,  GE,

    ARROW,   BOX,

    ID { id :  String },
    NUM{ val : String },
    EOF,
}

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
    pub fn scan(&mut self) -> Token {
        match self.put_back {
            true => { self.put_back = false; },
            false => { self.curr_ch = self.next_char(); }
        }

        match self.curr_ch {
            None     => { Token{ line : self.line, typ : TokenType::EOF } },
            Some(ch) => {
                if ch.is_alphabetic() {
                    Token { line : self.line, 
                            typ : TokenType::ID{id : self.build_val(|c| c.is_alphabetic()) }} 
                } else if ch.is_numeric() {
                    Token { line : self.line, 
                            typ : TokenType::NUM{val : self.build_val(|c| c.is_numeric()) }} 
                } else {
                    self.process_special()
                }
            }
        }
    }

    pub fn build_val<F>(&mut self, func : F) -> String
        where F : Fn(char) -> bool { 
        
        let id = vec![];
        loop {
            let ch = match self.curr_ch {
                Some(ch) => ch,
                None => break,
            };
            match F(ch) { 
                true => id.push(ch),
                false => break,
            };
        }
        self.put_back = true;
        id.iter().map(|c| *c).collect()
    }
}
