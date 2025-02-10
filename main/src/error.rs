#[derive(Debug)]
pub struct BBError{
    line: usize,
    message:String,
}

impl BBError{

pub fn error(line: usize,message: String)-> BBError{
    BBError {line,message}
}
pub fn report(&self,loc: String){
    eprintln!("[line{}] Error{}: {}", self.line,loc,self.message );
}
}