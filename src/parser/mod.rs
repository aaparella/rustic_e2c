pub mod scanner;
pub mod token;
pub mod symbol_table;

use self::scanner::Scanner;
use self::token::{Token, TokenType};
use self::symbol_table::SymbolTable;
use std::io::Write;
use std::io;

pub struct Parser {
    token    : Token,
    scanner  : Scanner,
    sym_tab  : SymbolTable,
}

impl Parser {
    pub fn new(filename : String) -> Parser {
        Parser {
            token    : Token { line : 0, typ : TokenType::EOF },
            scanner  : Scanner::new(&filename),
            sym_tab  : SymbolTable::new(),
        }   
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
        
        self.sym_tab.display_variables();
    }

    // program ::= block
    fn program(&mut self) {
        println!("#include <stdio.h>\n");
        println!("int main()\n{{");
        self.block();
        println!("return 0;\n}}");
    }

    // block ::= [declarations] statement_list
    fn block(&mut self) {
        self.sym_tab.add_frame();
        if self.token_match(TokenType::VAR) {
            self.declarations();
        }
        self.statement_list();
        self.sym_tab.pop_frame();
    }

    // declarations ::= "var" { id } "rav"
    fn declarations(&mut self) {
        self.must_be(TokenType::VAR);
        while self.token_match(TokenType::ID("".to_string())) {
            match self.sym_tab.declared_in_block(&self.token) {
                true  => println!("[WARNING] Redeclared variable {:?}", self.token),
                false =>  {
                    match self.token.typ {
                        TokenType::ID(ref id) => println!("int x_{}=-12345;", id),
                        _ => panic!("[ERROR] Uh oh"),
                    };
                    self.sym_tab.add_var(&self.token);
                }
            };
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

    // statement ::= assignment | if | do | fa | print
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

    // assignment ::= id ":=" expression
    fn assignment(&mut self) {
        if !self.sym_tab.in_scope(&self.token) {
            panic!("[ERROR] Assigning to undeclared ID {:?}", self.token);
        }
        self.sym_tab.inc_assign(&self.token);
        println!("");
        self.must_be(TokenType::ID("".to_string()));
        self.must_be(TokenType::ASSIGN);
        self.expression();
    }

    // print ::= "print" expression
    fn print(&mut self) {
        self.must_be(TokenType::PRINT);
        self.expression();
    }
    
    // if ::= "if" guarded_commands "fi"
    fn eif(&mut self) {
        self.must_be(TokenType::IF);
        self.guarded_commands();
        self.must_be(TokenType::FI);
    }

    // do ::= "do" guarded_commands "od"
    fn edo(&mut self) {
        self.must_be(TokenType::DO);
        self.guarded_commands();
        self.must_be(TokenType::OD);
    }

    // fa ::= "fa" id ":=" expression "to" expression ["st" expression] commands "af"
    fn fa(&mut self) {
        self.must_be(TokenType::FA);
        if !self.sym_tab.in_scope(&self.token) {
            panic!("[ERROR] Reference to undeclared ID {:?}", self.token);
        }
        self.sym_tab.inc_assign(&self.token);
        self.must_be(TokenType::ID("".to_string()));
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

    // guarded_commands ::= guarded_command { "[]" guarded_command } [ "else" commands ]
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

    // guarded_command ::= expression commands
    fn guarded_command(&mut self) {
        self.expression();
        self.commands();
    }

    // commands ::= "->" block
    fn commands(&mut self) {
        self.must_be(TokenType::ARROW);
        self.block();
    }

    // expression ::= simple [relop simple]
    fn expression(&mut self) {
        self.simple();
        if self.is_relop() {
            self.relop();
            self.simple();
        }
    }

    // simple ::= term {addop term}
    fn simple(&mut self) {
        self.term();
        while self.is_addop() {
            self.addop();
            self.term();
        }
    }

    // term ::= factor { multop factor }
    fn term(&mut self) {
        self.factor();
        while self.is_multop() {
            self.multop();
            self.factor();
        }
    }

    // factor ::= "(" expression ")" | id | number | "^" expression | "@" expression
    // TODO : Implement sqrt and power operators
    fn factor(&mut self) {
        match self.token.typ {
            TokenType::ID(_) => { 
                if !self.sym_tab.in_scope(&self.token) {
                    panic!("[ERROR] Reference to undeclared variable {:?}", self.token);
                }
                self.sym_tab.inc_usage(&self.token);
                self.must_be(TokenType::ID("".to_string()));
            },
            TokenType::NUM(_) => { self.must_be(TokenType::NUM("".to_string())); },
            TokenType::LPAREN => {
                self.must_be(TokenType::LPAREN);
                self.expression();
                self.must_be(TokenType::RPAREN);
            }, 
            _ => self.error("factor"),
        };
    }

    // relop ::= "=" | "<" | ">" | "/=" | "<=" | ">="
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

    // addop ::= "+" | "-"
    fn addop(&mut self) {
        match self.token.typ {
            TokenType::PLUS  => self.must_be(TokenType::PLUS),
            TokenType::MINUS => self.must_be(TokenType::MINUS),
            _ => self.error("addop"),
        };
    }
    
    // multop ::= "*" | "/"
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
    fn must_be(&mut self, typ : TokenType) {
         match typ {
            TokenType::ID(_) => {
                match self.token.typ {
                    TokenType::ID(_) => self.scan(),
                    _ => self.error("ID"),
                }
            },
            TokenType::NUM(_) => {
                match self.token.typ {
                    TokenType::NUM(_) => self.scan(),
                    _ => self.error("NUM"),
                }
            },
            _ => {
                match self.token.typ == typ {
                    true => self.scan(),
                    false => self.error("Something"),
                }
            },
        };
    }        

    // Return true if current token is of specified type, false if not
    fn token_match(&self, typ : TokenType) -> bool {
        match typ {
            TokenType::ID(_) => {
                match self.token.typ {
                    TokenType::ID(_) => true,
                    _ => false,
                }
            },
            TokenType::NUM(_) => {
                match self.token.typ {
                    TokenType::NUM(_) => true,
                    _ => false,
                }
            },
            _ => self.token.typ == typ,
        }
    }
}    


