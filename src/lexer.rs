use crate::tokens::Tokens;

pub struct Lexer {}

impl Lexer {
    pub fn lex(source: String) -> Vec<(Tokens, String)> {
        let mut tokens: Vec<(Tokens, String)> = Vec::new();
        let mut _lines = source.lines();
        
        let delimiters: [(Tokens, char) ; 6] = [
            (Tokens::Equal, '='),
            (Tokens::Colon, ':'),
            (Tokens::LParen, '('),
            (Tokens::RParen, ')'),
            (Tokens::LCurlyBrace, '{'),
            (Tokens::RCurlyBrace, '}'),
        ];
        
        let keywords: [(Tokens, &str) ; 3] = [
            (Tokens::Var, "var"),
            (Tokens::Function, "fn"),
            (Tokens::Return, "return"),
        ];
        
        let mut tok: String = String::new();
        for mut line in _lines {
            line = line.trim();
            // Handle delims
            'char_loop: for c in line.chars() {
                for delim in delimiters {
                    if c == delim.1 {
                        if !tok.is_empty() {
                            match tok.parse::<i64>() {
                                Ok(_) => {
                                    tokens.push((Tokens::Number, tok.clone()));
                                    tok.clear();
                                }
                                
                                Err(_) => {
                                    tokens.push((Tokens::None, tok.clone()));
                                    tok.clear();
                                }
                            }
                        }
                        
                        tokens.push((delim.0, delim.1.to_string()));
                        continue 'char_loop;
                    }
                }
                
                for keyword in keywords {
                    if tok == keyword.1 {
                        tokens.push((keyword.0, keyword.1.to_string()));
                        tok.clear();
                        continue 'char_loop;
                    }
                }
                
                if c.is_whitespace() || c == ';' {
                    if !tok.is_empty() {
                        match tok.parse::<i64>() {
                            Ok(_) => {
                                tokens.push((Tokens::Number, tok.clone()));
                                tok.clear();
                            }
                            
                            Err(_) => {
                                tokens.push((Tokens::None, tok.clone()));
                                tok.clear();
                            }
                        }
                    }
                } else {
                    tok.push(c);
                }
            }
        }
        
        let mut index = 0;
        while index < tokens.len() {
            for keyword in keywords {
                if index > 0 && tokens[index].0 == Tokens::None {
                    tokens[index].0 = Tokens::Identifier;
                }
            }
            
            index += 1;
        }
        
        tokens
    }
}