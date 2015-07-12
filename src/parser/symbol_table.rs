use super::token::{Token, TokenType};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Variable {
    name  : String,
    declared : usize,
    uses  : HashMap<usize, usize>,
    assignments : HashMap<usize, usize>, 
    depth : usize,
}

// Make it much easier to check for a variable
impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// Yeah, it sucks, I'll fix it 
impl fmt::Display for Variable {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{}\n", self.name);
        let _ = write!(f, "\tdeclared on {} at depth {}\n\tUses : ", self.name, self.depth);
        for key in self.uses.keys() {
            let _ = match self.uses.get(key).unwrap() {
                &1 => write!(f, "{} ", key),
                _ => write!(f, "{}({}) ", key, self.uses.get(key).unwrap()),
            };
        }
        let _ = write!(f, "\n\tAssignments : ");
        for key in self.assignments.keys() {
            let _ = match self.assignments.get(key).unwrap() {
                &1 => write!(f, "{} ", key),
                _ => write!(f, "{}({})", key, self.assignments.get(key).unwrap()),
            };
        }
        write!(f, "\n")
    }
}

impl Variable {
    // Cosntruct a Variable from a token
    // Will almost strictly be used
    pub fn from_token(token : &Token, depth : usize) -> Variable {
        Variable {
            name : match token.typ {
                    TokenType::ID(ref id)   => id.chars().collect(),
                    TokenType::NUM(ref num) => num.chars().collect(),
                    _ => panic!("[ERROR] Tried to convert non ID / NUM to variable") },
            uses  : HashMap::new(),
            assignments : HashMap::new(), 
            declared : token.line,
            depth : depth
        }
    }

    pub fn inc_usage(&mut self, line : usize) {
        let uses = self.uses.entry(line).or_insert(0);
        *uses += 1;
    }

    pub fn inc_assignment(&mut self, line : usize) {
        let assignments = self.assignments.entry(line).or_insert(0);
        *assignments += 1;
    }
}

pub struct SymbolTable {
    frames : Vec< Vec<Variable> >,
    vars   : Vec< Variable >,
    depth  : usize,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable{ frames : vec![], vars : vec![],  depth : 0 }
    }

    pub fn add_frame(&mut self) {
        self.frames.push(vec![]);
        self.depth += 1;
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
        self.depth -= 1;
    }

    pub fn add_var(&mut self, tok : &Token) {
        self.frames.last_mut().unwrap().push(Variable::from_token(tok, self.depth));
        self.vars.push(Variable::from_token(tok, self.depth));
    }

    pub fn inc_usage(&mut self, tok : &Token) {
        let var = Variable::from_token(tok, self.depth);
        for v in self.vars.iter_mut().rev() {
            if *v == var {
                v.inc_usage(tok.line);
                break;
            }
        }
    }

    pub fn inc_assign(&mut self, tok : &Token) {
        let var = Variable::from_token(tok, self.depth);
        for v in self.vars.iter_mut().rev() {
            if *v == var {
                v.inc_assignment(tok.line);
                break;
            }
        }
    }
    
    pub fn in_scope(&self, tok : &Token) -> bool {
        self.frames.iter().any(|frame| (*frame).contains(&Variable::from_token(tok, self.depth)))
    }
    
    pub fn declared_in_block(&self, tok : &Token) -> bool {
        let frame = self.frames.last().unwrap();
        let var = Variable::from_token(tok, self.frames.len());
        frame.iter().any(|v| *v == var)
    }

    pub fn display_variables(&mut self) {
        for var in &self.vars {
            println!("{}", var);
        }
    }
}
