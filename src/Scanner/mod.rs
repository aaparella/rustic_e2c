use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Token {
    typ  : TokenType,
    line : u16,
}

#[derive(Debug)]
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

fn type_for_id(_id : String) -> TokenType {
    match _id.as_ref() {
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
         _ => TokenType::ID{id : _id},
     }
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

    pub fn process(&mut self) {
        loop {
            let tok = self.scan();
            match tok.typ {
                TokenType::EOF => break,
                _ => println!("{:?}", tok),
            };
        }
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
                } else if ch == ' ' || ch == '\t'{
                    self.scan() 
                } else if ch == '\n' {
                    self.line += 1;
                    self.scan()
                } else if ch.is_alphabetic() {
                    let _id = self.build_val(|c| c.is_alphabetic() );
                    Token { line : self.line, typ : type_for_id(_id) }
                } else if ch.is_numeric() {
                    Token { line : self.line, 
                            typ : TokenType::NUM{val : self.build_val(|c| c.is_numeric()) }} 
                } else {
                    Token { line : self.line,
                            typ : self.process_special() }
                }
            }
        }
    }

    fn process_special(&mut self) -> TokenType {
        let ch = self.curr_ch.unwrap();
        match ch {
            '('  => TokenType::LPAREN,
            ')'  => TokenType::RPAREN,
            '='  => TokenType::EQ,
            '+'  => TokenType::PLUS,
            '-'  => TokenType::MINUS,
            '>'  => self.next_might_be('=', TokenType::LT, TokenType::LE),
            '<'  => self.next_might_be('=', TokenType::GT, TokenType::GE),
            '\\' => self.next_might_be('=', TokenType::DIVIDE, TokenType::NE),
            ':'  => self.next_must_be(':', TokenType::ASSIGN),
            _ => panic!("[ERROR] Unsupported character {} on line {}", ch, self.line),
        }
    }

    fn next_might_be(&mut self, next : char, if_one : TokenType, if_two : TokenType) -> TokenType {
        let ch = self.next_char().unwrap();
        match ch == next {
            true => if_two, 
            false =>  { self.put_back = true; if_one },
        }
    }

    fn next_must_be(&mut self, next : char, typ : TokenType) -> TokenType {
        let ch = self.next_char().unwrap();
        match ch == next {
            true => typ,
            false => panic!("[ERROR] On line {}, expected {} found {}", self.line, next, ch),
        }
    }

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
        }
        self.put_back = true;
        id.iter().map(|c| *c).collect()
    }
}
