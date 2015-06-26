use std::env;
mod scanner;
use scanner::{Scanner, Token, TokenType};

struct Parser<'a> {
    token : Token,
    scanner : &'a mut Scanner,
}

impl<'a> Parser<'a> {
    pub fn new(scanner : &'a mut Scanner) -> Parser {
        Parser { token : Token { line : 0, typ : TokenType::EOF } , scanner : scanner }
    }

    fn error(&self, foo : &str) -> ! {
        panic!("[ERROR] Could not parse {}", foo);
    }

    // Parse through the file given to the provided Scanner to tokenize
    pub fn parse(&mut self) {
        self.scan();
        self.program();
       
        if !self.token_match(TokenType::EOF) {
            panic!("[ERROR] Junk after logical end of program");
        }
    }

    // program ::= block
    fn program(&mut self) {
        self.block();
    }

    // block ::= [declarations] statement_list
    fn block(&mut self) {
        if self.token_match(TokenType::VAR) {
            self.declarations();
        }

        self.statement_list();
    }

    // declarations ::= "var" { id } "rav"
    fn declarations(&mut self) {
        self.must_be(TokenType::VAR);
        while self.token_match_id() {
            self.scan();
        }
        self.must_be(TokenType::RAV);
    }

    // { statement } 
    fn statement_list(&mut self) {
        while self.is_statement() {
            self.statement();
        }
    }

    fn statement(&mut self) {
        match self.token.typ {
            TokenType::ID(_) => self.assignment(),
            TokenType::IF    => self.eif(),
            TokenType::DO    => self.edo(),
            TokenType::FA    => self.fa(),
            TokenType::PRINT => self.print(),
            _ => self.error("statement"),
        };
     } 

    fn assignment(&mut self) {
        self.must_be_id();
        self.must_be(TokenType::ASSIGN);
        self.expression();
    }

    fn print(&mut self) {
        self.must_be(TokenType::PRINT);
        self.expression();
    }
    
    fn eif(&mut self) {
        self.must_be(TokenType::IF);
        self.guarded_commands();
        self.must_be(TokenType::FI);
    }

    fn edo(&mut self) {
        self.must_be(TokenType::DO);
        self.guarded_commands();
        self.must_be(TokenType::OD);
    }

    fn fa(&mut self) {
        self.must_be(TokenType::FA);
        self.must_be_id();
        self.must_be(TokenType::ASSIGN);
        self.expression();
        self.must_be(TokenType::TO);
        self.expression();

        if self.token_match(TokenType::ST) {
            self.must_be(TokenType::ST);
            self.expression();
        }

        self.commands();
        self.must_be(TokenType::AF);
    }

    fn guarded_commands(&mut self) {
        self.guarded_command();
        while self.token_match(TokenType::BOX) {
            self.must_be(TokenType::BOX);
            self.guarded_command();
        }

        if self.token_match(TokenType::ELSE) {
            self.must_be(TokenType::ELSE);
            self.commands();
        }
    }

    fn guarded_command(&mut self) {
        self.expression();
        self.commands();
    }

    fn commands(&mut self) {
        self.must_be(TokenType::ARROW);
        self.block();
    }

    fn expression(&mut self) {
        self.simple();
        if self.is_relop() {
            self.relop();
            self.simple();
        }
    }

    fn simple(&mut self) {
        self.term();
        while self.is_addop() {
            self.addop();
            self.term();
        }
    }

    fn term(&mut self) {
        self.factor();
        while self.is_multop() {
            self.multop();
            self.factor();
        }
    }

    fn factor(&mut self) {
        match self.token.typ {
            TokenType::ID(_) => { self.must_be_id(); },
            TokenType::NUM(_) => { self.must_be_num(); },
            TokenType::LPAREN => {
                self.must_be(TokenType::LPAREN);
                self.expression();
                self.must_be(TokenType::RPAREN);
            }, 
            _ => self.error("factor"),
        };
   }

    fn relop(&mut self) {
        match self.token.typ {
            TokenType::EQ => self.must_be(TokenType::EQ),
            TokenType::LT => self.must_be(TokenType::LT),
            TokenType::GT => self.must_be(TokenType::GT),
            TokenType::NE => self.must_be(TokenType::NE),
            TokenType::LE => self.must_be(TokenType::LE),
            TokenType::GE => self.must_be(TokenType::GE),
            _ => self.error("relop"),
        };
    }

    fn addop(&mut self) {
        match self.token.typ {
            TokenType::PLUS  => self.must_be(TokenType::PLUS),
            TokenType::MINUS => self.must_be(TokenType::MINUS),
            _ => self.error("addop"),
        };
    }
    
    fn multop(&mut self) {
        match self.token.typ {
            TokenType::TIMES  => self.must_be(TokenType::TIMES),
            TokenType::DIVIDE => self.must_be(TokenType::DIVIDE),
            _ => self.error("multop"),
        };
    }

    // Just to avoid having to type out self.scanner.scan()
    fn scan(&mut self) {
        self.token = self.scanner.scan();
    }

    fn is_multop(&self) -> bool {
        match self.token.typ {
            TokenType::DIVIDE | TokenType::TIMES => true,
            _ => false,
        }
    }

    fn is_relop(&self) -> bool {
        match self.token.typ {
            TokenType::NE | TokenType::LT | TokenType::GT | TokenType::EQ | TokenType::LE | TokenType::GE => true,
            _ => false,
        }
    }

    fn is_addop(&self) -> bool {
        match self.token.typ {
            TokenType::PLUS | TokenType::MINUS => true,
            _ => false,
        }
    }

    fn is_statement(&self) -> bool {
        match self.token.typ {
            TokenType::ID(_) | TokenType::PRINT | TokenType::IF | TokenType::DO | TokenType::FA => true,
            _ => false,
        }
    }

    // Checks if current TokenType is equal to that specified
    fn token_match(&self, typ : TokenType) -> bool {
        self.token.typ == typ
    }

    // This disgusts me
    fn token_match_id(&self) -> bool {
        match self.token.typ {
            TokenType::ID(_) => true,
            _ => false,
        }
    }

    // Ew
    fn token_match_num(&self) -> bool {
        match self.token.typ {
            TokenType::NUM(_) => true,
            _ => false,
        }
    }

    // I threw up a bit inside
    fn must_be_id(&mut self) {
        match self.token.typ {
            TokenType::ID(_) => self.scan(),
            _ => panic!("[ERROR] Something something"),
        };
    }
    
    // Someone save me
    fn must_be_num(&mut self) {
        match self.token.typ {
            TokenType::NUM(_) => self.scan(),
            _ => panic!("[ERROR] Expected TokenType::ID found {:?} on line {}", self.token.typ, self.token.line),
        };
    }

    // Panics if the current TokenType is not equal to that specified
    fn must_be(&mut self, typ : TokenType) {
        match self.token.typ == typ {
            true => { self.scan() },
            false => panic!("[ERROR] Expected {:?} found {:?} on line {}", typ, self.token.typ, self.token.line),
        };
    }
}    

fn main() {
    let mut scanner = Scanner::new(env::args().nth(1).unwrap());
    let mut parser = Parser::new(&mut scanner);
    parser.parse();
}
