use std::fs::File;

pub struct Scanner {
    contents : String,
    position : u32,
}

impl Scanner { 
    pub fn new(filename : String) -> Scanner {
        let file = match File::open(&filename) {
            Ok(f) => f,
            Err(e) => panic!("Could not open {} : {}", filename, e),
        };
        let cont = String::new();
        Scanner { contents : cont, position : 0 }
    }

    
}
