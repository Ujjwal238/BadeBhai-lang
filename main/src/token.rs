use std::fmt;
use crate:: token_type::*; 
#[derive(Debug)]
pub enum Object{
    Num(f64),
    Str(String),
    Nalla,
    Sahi,
    Galat,
    
}
impl fmt::Display for Object{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
     match self{
         Object::Str(x)=> write!(f,"\"{x}\""),
         Object::Nalla=> write!(f,"nalla"),
         Object::Sahi=> write!(f,"sahi"),
         Object::Galat=> write!(f,"galat"),
         Object::Num(x)=> write!(f,"{x}"),
     }
    }
}
#[derive(Debug)]
pub struct Token{
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,

}
impl Token{
    pub fn new(ttype: TokenType,lexeme:String,literal:Option<Object>,line: usize)-> Token{
        Token{ttype,lexeme,literal,line,}
    }

pub fn eof(line:usize)-> Token{
    Token{ ttype:TokenType::Eof,lexeme:"".to_string(), literal:None,line}
}
}
impl fmt:: Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        write!(f,"{:?} {} {}", self.ttype,self.lexeme, if let Some(literal)= &self.literal{
                literal.to_string()
        } else {
            "None".to_string()
        }
    )
    }
} 

// pub enum Token {
//     Literal {lexeme: String, literal:<Object>},
//     Keyword (lexeme: String, ttype: String),

// }