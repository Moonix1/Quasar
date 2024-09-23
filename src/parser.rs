use std::ops::Add;

use crate::tokens::Tokens;

use log::*;

pub struct Parser {}

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Number(i64),
    Binary {
        left: Box<Expr>,
        operator: Tokens,
        right: Box<Expr>
    },
    Variable {
        name: String,
        vtype: String,
        value: Box<Expr>,
    },
    Function {
        name: String,
        args: Vec<Expr>,
        rtype: String,
        body: Vec<Expr>,
    },
}

impl Parser {
    fn parse_parameters(tokens: Vec<(Tokens, String)>, parameters: &mut Vec<Expr>) -> Vec<Expr> {
        let mut pexpr = Vec::<Expr>::new();
        
        for tok in tokens {
            match tok.0 {
                _ => {}
            }
        }
        
        pexpr
    }
    
    pub fn parse(tokens: Vec<(Tokens, String)>) -> Vec<Expr> {
        let mut expr = Vec::<Expr>::new();
        
        let mut tok_i = 0;
        while tok_i < tokens.len() {
            let tok = tokens[tok_i].clone();
            match tok.0 {
                Tokens::Function => {
                    let mut fname = String::new();
                    let mut parameters = Vec::<Expr>::new();
                    let mut ftype = String::from("void");
                    
                    fname = Self::next(tokens.clone(), &mut tok_i).1;
                    
                    Self::next(tokens.clone(), &mut tok_i);
                    if tokens[tok_i].0 != Tokens::LParen {
                        error!("Missing `(` after function identifier");
                    }
                    let lparen = tok_i;
                    Self::get_n(Tokens::RParen, tokens.clone(), &mut tok_i);
                    tok_i += 1;
                    if tokens[tok_i].0 != Tokens::RParen {
                        error!("Missing closing delimiter `)` after parameters");
                    }
                    let rparen = tok_i;
                    
                    if Self::next(tokens.clone(), &mut tok_i).0 == Tokens::Colon {
                        ftype = Self::next(tokens.clone(), &mut tok_i).1;
                    }
                    
                    expr.push(Expr::Function {
                        name: fname, args: Self::parse_parameters(tokens[lparen..rparen].to_vec(), &mut parameters), rtype: ftype, body: Vec::new()
                    });
                }
                
                _ => {
                    error!("Invalid token `{}`", tok.1);
                }
            }
            
            tok_i += 1;
        }
        
        expr
    }
    
    fn next(tokens: Vec<(Tokens, String)>, tok_i: &mut usize) -> (Tokens, String) {
        *tok_i += 1;
        tokens[*tok_i].clone()
    }
    
    fn get_n(tok: Tokens, tokens: Vec<(Tokens, String)>, tok_i: &mut usize) -> (Tokens, String) {
        let mut index = *tok_i;
        while index < tokens.len() {
            let tok = tokens[index].clone();
            if tok.0 == tok.0 {
                return tok;
            }
            
            index += 1;
        }
        
        (Tokens::None, "?".to_string())
    }
}