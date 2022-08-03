use token::TokenType;
use std::fmt;
#[derive(Debug,Clone)]
pub struct Parser {
   tokens:Vec<TokenType>, 
}

impl Parser {
    pub fn new (t:Vec<TokenType>) -> Self {
        Self {
            tokens:t
        }
    }
    pub fn peek(&self) -> TokenType {
        self.tokens.to_vec().into_iter().next().unwrap()
    }  
    pub fn nom (self) -> TokenType { // its the eat/consume function but I think its funnier to call it nom
        self.tokens.to_vec().into_iter().next().unwrap()
    }
}
#[derive(Debug,Clone)]
pub struct Astnode {
    value:String,     
    children:Vec<Astnode>,
}
pub struct Ast {
    tree:Vec<Astnode>
}
impl fmt::Display for Ast {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       for node in self.tree.to_vec(){
           writeln!(f,"{}", node.value);
           for child in node.children {
               if node.children.to_vec().len() == 1 {
               }
           }
           //writeln!(f,"{}",format!("{}",node.value)) 
       }
       Ok(())  
     }
}
