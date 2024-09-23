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

pub fn get_tok(scope_index: u64, find: Tokens, tokens: &Vec<(u64, Tokens, String)>, index: &usize) -> (u64, Tokens, String) {
    let mut current_index = 0;
    for token in tokens {
        if current_index > *index {
            if find == token.1 {
                return token.clone();
            }
        }
        
        current_index += 1;
    }
    
    (0, Tokens::None, String::new())
}

pub fn find(scope_index: u64, find: &str, tokens: &Vec<(u64, Tokens, String)>, index: &usize) -> bool {
    let mut current_index = 0;
    for token in tokens {
        if current_index > *index {
            if token.2 == find && token.0 == scope_index {
                return true;
            }
        }
        
        current_index += 1;
    }
    
    false
}

pub fn find_tok(scope_index: u64, find: Tokens, tokens: &Vec<(u64, Tokens, String)>, index: &usize) -> bool {
    let mut current_index = 0;
    for token in tokens {
        if current_index > *index {
            if token.1 == find && token.0 == scope_index {
                return true;
            }
        }
        
        current_index += 1;
    }
    
    false
}

pub fn find_i(scope_index: u64, find: &str, tokens: &Vec<(u64, Tokens, String)>, index: &usize) -> usize {
    let mut current_index = 0;
    for token in tokens {
        if current_index > *index && token.0 == scope_index {
            if token.2 == find {
                return current_index;
            }
        }
        
        current_index += 1;
    }
    
    current_index
}

pub fn find_tok_i(scope_index: u64, find: Tokens, tokens: &Vec<(u64, Tokens, String)>, index: &usize) -> usize {
    let mut current_index = 0;
    for token in tokens {
        if current_index > *index && token.0 == scope_index {
            if token.1 == find {
                return current_index;
            }
        }
        
        current_index += 1;
    }
    
    current_index
}

pub fn find_last_tok_i(scope_index: u64, find: Tokens, tokens: &Vec<(u64, Tokens, String)>, index: &usize) -> usize {
    let mut current_index = 0;
    while current_index <= tokens.len() {
        let token = tokens[current_index].clone();
        if token.0 == scope_index {
            if token.1 == find {
                return current_index;
            }
        }
        
        current_index -= 1;
    }
    
    current_index
}

pub fn find_last_tok(scope_index: u64, find: Tokens, tokens: &Vec<(u64, Tokens, String)>, index: &usize) -> bool {
    let mut current_index = 0;
    while current_index <= tokens.len() {
        let token = tokens[current_index].clone();
        if token.0 == scope_index {
            if token.1 == find {
                return true;
            }
        }
        
        current_index -= 1;
    }
    
    false
}