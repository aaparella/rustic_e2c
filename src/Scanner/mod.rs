use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Token {
    pub typ  : TokenType,
    pub line : u16,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    VAR,   RAV,   PRINT,
    IF,    FI,    DO,
    OD,    ELSE,  FA,
    AF,    TO,    ST,

    ASSIGN,    LPAREN,
    RPAREN,    PLUS,
    MINUS,     TIMES,    DIVIDE,
    SQUARE,    SQRT,
    EQ,  NE,  LT,
    GT,  LE,  GE,

    ARROW,   BOX,

    ID(String),
    NUM(String),
    EOF, UNSUP(char),
}

pub struct Scanner {
    contents : String,
    curr_ch  : Option<char>,
    position : usize,
    line     : u16,
    put_back : bool,
}

// Get the approprite type for a given ID
fn type_for_id(id : String) -> TokenType {
    match id.as_ref() {
        "var" => TokenType::VAR,
        "rav" => TokenType::RAV,
        "print" => TokenType::PRINT,
        "if" => TokenType::IF,
        "fi" => TokenType::FI,
        "do" => TokenType::DO,
        "od" => TokenType::OD,
        "else" => TokenType::ELSE,
        "fa" => TokenType::FA,
        "af" => TokenType::AF,
        "to" => TokenType::TO,
        "st" => TokenType::ST,
         _ => TokenType::ID(id),
     }
}

impl Scanner { 
    // Create a new scanner, scanning contents of file filename
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
                  line     : 1, 
                  put_back : false}
    }

    // Advance character by one, DOES NOT set curr_ch
    fn next_char(&mut self) -> Option<char> {
        self.position += 1;
        self.contents.chars().nth(self.position - 1)
    }

    // Simple dummy function for testing functionality
    pub fn process(&mut self) {
        loop {
            let tok = self.scan();
            match tok.typ {
                TokenType::EOF => { println!("{:?}", tok); break; },
                _ => println!("{:?}", tok),
            };
        }
    }

    // Return the next Token in the file
    // Unsupported characters / EOF are treated as Tokens, no panicing
    pub fn scan(&mut self) -> Token {
        match self.put_back {
            true => { self.put_back = false; },
            false => { self.curr_ch = self.next_char(); }
        }

        match self.curr_ch {
            None     => { Token{ line : self.line, typ : TokenType::EOF } },
            Some(ch) => {
                // Chrew through a commented line
                if ch == '#' {
                    loop {
                        let c = self.next_char();
                        match c {
                            None => return Token { line : self.line, typ : TokenType::EOF },
                            Some(c) => {
                                if c == '\n' { break; }
                            }
                        }
                    }
                    self.line += 1;
                    self.scan()
                } else if ch == ' ' || ch == '\t' {
                    self.scan() 
                } else if ch == '\n' {
                    self.line += 1;
                    self.scan()
                } else if ch.is_alphabetic() {
                    let _id = self.build_val(|c| c.is_alphabetic() );
                    Token { line : self.line, typ : type_for_id(_id) }
                } else if ch.is_numeric() {
                    Token { line : self.line, 
                            typ : TokenType::NUM(self.build_val(|c| c.is_numeric())) } 
                } else {
                    Token { line : self.line,
                            typ : self.process_special() }
                }
            }
        }
    }

    // Match each special character to approprite TokenType
    fn process_special(&mut self) -> TokenType {
        let ch = self.curr_ch.unwrap();
        match ch {
            '('  => TokenType::LPAREN,
            ')'  => TokenType::RPAREN,
            '='  => TokenType::EQ,
            '+'  => TokenType::PLUS,
            '*'  => TokenType::TIMES,
            '@'  => TokenType::SQRT,
            '^'  => TokenType::SQUARE,
            '>'  => self.next_might_be('=', TokenType::GT, TokenType::LE),
            '-'  => self.next_might_be('>', TokenType::MINUS, TokenType::ARROW),
            '<'  => self.next_might_be('=', TokenType::LT, TokenType::GE),
            '\\' => self.next_might_be('=', TokenType::DIVIDE, TokenType::NE),
            ':'  => self.next_must_be('=', TokenType::ASSIGN),
            '['  => self.next_must_be(']', TokenType::BOX),
            _    => TokenType::UNSUP(ch),
        }
    }

    // Current character may or may not be followed by next
    // If it is the two character sequence, return if_two, if not, if_one
    fn next_might_be(&mut self, next : char, if_one : TokenType, if_two : TokenType) -> TokenType {
        self.curr_ch = self.next_char();
        match self.curr_ch.unwrap() == next {
            true => if_two, 
            false =>  { 
                self.put_back = true; 
                if_one },
        }
    }

    // The current character MUST be followed by next, otherwise we panic
    fn next_must_be(&mut self, next : char, typ : TokenType) -> TokenType {
        self.curr_ch = self.next_char();
        match self.curr_ch.unwrap() == next {
            true => typ,
            false => panic!("[ERROR] On line {}, expected {} found {}", self.line, next, self.curr_ch.unwrap()),
        }
    }

    // Build the value for either an ID or a numeric
    // Keep taking characters so long as func is true for each
    fn build_val<F>(&mut self, func : F) -> String
        where F : Fn(char) -> bool { 
        
        let mut id = vec![];
        loop {
            let ch = match self.curr_ch {
                Some(ch) => ch,
                None => break,
            };
            match func(ch) { 
                true => id.push(ch),
                false => break,
            };
            self.curr_ch = self.next_char();
        }
        // We've gone one past the ID, so put back last character
        self.put_back = true;
        id.iter().map(|c| *c).collect()
    }
}
