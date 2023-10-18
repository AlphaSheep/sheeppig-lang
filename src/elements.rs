#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Simple(String),
    Compound(Vec<String>),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Float(f64),
    Integer(i64),
    Char(char),
    String(String),
    Boolean(bool),
    None,
}


impl Literal {
    pub fn from_str(literal: &str) -> Option<Literal> {
        match literal {
            "true" => Some(Literal::Boolean(true)),
            "false" => Some(Literal::Boolean(false)),
            "None" => Some(Literal::None),
            _ => None,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic operators
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Power,

    // Logical operators
    And,
    Or,
    Not,

    // Bitwise operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseNot,

    // Relational operators
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Using,
    As,
    From,

    Function,
    Return,

    Variable,

    If,
    Else,

    For,
    In,
    While,
}


impl Keyword {
    pub fn from_str(keyword: &str) -> Option<Keyword> {
        match keyword {
            "using" => Some(Keyword::Using),
            "as" => Some(Keyword::As),
            "from" => Some(Keyword::From),

            "fun" => Some(Keyword::Function),
            "return" => Some(Keyword::Return),

            "var" => Some(Keyword::Variable),

            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),

            "for" => Some(Keyword::For),
            "in" => Some(Keyword::In),
            "while" => Some(Keyword::While),

            _ => None,
        }
    }
}
