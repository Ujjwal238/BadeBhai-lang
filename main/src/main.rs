// chapter 4- scanning interpreter
use std::fs::File;
use std::io::{self,BufRead,BufReader,Read};
use std::env::args;
pub fn main(){
    let args: Vec<String> = args().collect();
    println!("args:{:?}",args);  //args is the name of the program
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

fn run_file(path:&String)-> io::Result<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf);

    run(&buf);
    Ok(());
}
fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty(){
                break;
            }  
            run(&line.as_bytes());
        }
        
       else {
           break;
       }
        
    }

}

fn run(source: &[u8]){}

