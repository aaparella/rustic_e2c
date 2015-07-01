#[derive(Debug)]
pub struct Variable {
    name  : String,
    uses  : u8,
    lines : Vec<u8>,
}


impl Variable {
    pub fn new(name : String, line : u8) -> Variable {
        Variable { name : name, uses : 1, lines : vec![line] }
    }

    pub fn inc_usage(&mut self, line : u8) {
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
}
