// chapter 4- scanning interpreter
use std::fs::File;
mod token;
mod scanner;
use scanner::*;
mod token_type;
mod error;
use error::*; 
use std::io::{self,BufRead,BufReader,Read};
use std::env::args;
pub fn main(){
    let args: Vec<String> = args().collect();
    // println!("args:{:?}",args);  //args is the name of the program
    if args.len() >2{
        println!("Usage: BadeBhai-ast [script]");
        std::process::exit(64);

    } 
    else if args.len()==1 {
        run_file(&args[1]).expect("Cannnot run file"); //whenpassing through as a pointer we give the next index
        

    }
    else {
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let buf = std:: fs:: read_to_string(path)?;
    match  run(buf) {
        Ok(_) =>{}
        Err(m) =>{
            m.report("".to_string());
            std::process::exit(65);
        }
       
    }
    Ok(())


    
}
fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty(){
                break;
            }  
            match run(line){
                Ok(_) => {}
                Err(m) => {
                    m.report("".to_string());
                }
            }

        }
        
       else {
           break;
       }
        
    }

}

fn run(source: String) -> Result<(),BBError>{
    let mut  scanner = Scanner::new (source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
        
    }
    Ok(())
}








