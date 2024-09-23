use inkwell::context::Context;
use std::{env, fs::File, io::Read, process::exit};

mod logger;
use logger::*;

use log::*;

mod tokens;
mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn get_file_contents(file_path: &str) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    
    contents
}

fn shift(args: &Vec<String>, args_index: &mut usize) -> String {
    let last_index = *args_index;
    *args_index += 1;
    args[last_index].clone()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args_index = 0;
    
    let logger = Logger::init();
    info!("Log initialized!");
    
    let program = shift(&args, &mut args_index);
    
    let flag = shift(&args, &mut args_index);
    match flag.as_str() {
        "-o" => {
            let input_file = shift(&args, &mut args_index);
            let output_file = shift(&args, &mut args_index);
            
            let contents = get_file_contents(input_file.as_str());
            let tokens = Lexer::lex(contents);
            let expr = Parser::parse(tokens.clone());
            println!("{:?}", tokens);
            println!("{:?}", expr);
        }
        
        _ => {
            error!("Invalid flag `{}`", flag);
            exit(1);
        }
    }
}