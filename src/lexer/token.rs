use std::fmt;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tokens {
   // Symbol
    Plus,
    Lparen,
    Rparen,
    Colon,
    ColonColon,
    Semi,
    Rbrack,
    Lbrack,
    RCbrack,
    LCbrack,
    

    // Identifier
    Int,
    Float,
    Bool,
    True,
    False,
    Fn,
    String,

    // Literals
    Literal(Literals),
}

pub enum Literals {
    Str(String),
    Int(i64),
}

impl Tokens {
    pub fn value(&self) -> Option<&'static str> {
        match self {
        Self::Plus => Some("+"),
        Self::Lparen => Some("("),
        Self::Rparen => Some(")"),
        Self::Colon => Some(":"),
        Self::ColonColon => Some("::"),
        Self::Semi => Some(";"),
        Self::Rbrack => Some("]"),
        Self::Lbrack => Some("["),
        Self::RCbrack => Some("}"),
        Self::LCbrack => Some("{"),

        Self::Int => Some("int"),
        Self::Float => Some("float"),
        Self::Bool => Some("bool"),
        Self::True => Some("true"),
        Self::False => Some("false"),
        Self::Fn => Some("func"),
        Self::String => None,

        Self::Literal => None,
        }
    }
}
impl fmt::Display for Tokens {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value().unwrap_or(""))
}
}
