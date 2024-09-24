#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Tokens {
    None,
    
    Var,
    Function,
    Return,
    
    Identifier,
    
    Number,
    
    Add,
    
    Equal,
    
    Colon,
    LParen,
    RParen,
    LCurlyBrace,
    RCurlyBrace,
    SemiColon,
}