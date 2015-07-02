use super::token::{Token, TokenType};

#[derive(Debug)]
pub struct Variable {
    name  : String,
    uses  : u16,
    lines : Vec<u16>,
}

// Make it much easier to check for a variable
impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.lines[0] == other.lines[0]
    }
}

impl Variable {
    pub fn new(name : String, line : u16) -> Variable {
        Variable { name : name, uses : 1, lines : vec![line] }
    }

    // Cosntruct a Variable from a token
    // Will almost strictly be used
    pub fn from_token(token : &Token) -> Variable {
        Variable {
            name : match token.typ {
                    TokenType::ID(ref id)   => id.chars().collect(),
                    TokenType::NUM(ref num) => num.chars().collect(),
                    _ => panic!("Shit") },
            uses  : 1, 
            lines : vec![token.line],
        }
    }

    pub fn inc_usage(&mut self, line : u16) {
        self.uses += 1;
        self.lines.push(line);
    }
}

pub struct SymbolTable {
    frames : Vec< Vec<Variable> >,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable{ frames : vec![] }
    }

    pub fn add_frame(&mut self) {
        self.frames.push(vec![]);
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn add_var(&mut self, var : Variable) {
        match self.frames.len() {
            0 => self.frames.push(vec![var]),
            _ => self.frames[0].push(var),
        };
    }

    pub fn table_contains(&self, var : Variable) -> bool {
        for frame in self.frames.iter() {
            if frame.contains(&var) {
                return true;
            }
        }
        false
    }
    
    pub fn declared(&self, var : Variable) -> bool {
        for frame in self.frames.iter().take(self.frames.len() - 1) {
            if frame.contains(&var) {
                return true;
            }
        }
        false
    }
}
