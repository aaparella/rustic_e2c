#[derive(Debug)]
pub struct Token {
    pub typ  : TokenType,
    pub line : usize,
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

// Get the approprite type for a given ID
pub fn type_for_id(id : String) -> TokenType {
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


