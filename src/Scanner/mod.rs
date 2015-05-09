pub struct scanner<'a> {
    x : &'a Vec<String>,
}

pub fn new(lines : &Vec<String>) -> scanner {
    scanner { x : lines }
}
