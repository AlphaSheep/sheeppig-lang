
use crate::elements::{Identifier, Literal, Operator};

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: Identifier,
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
    pub statements: StatementBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub name: Identifier,
    pub alias: Identifier,
    pub source: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Identifier>,
    pub body: Box<StatementBlock>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub param_type: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatementBlock {
    pub statements: Vec<Statement>,
}

impl StatementBlock {
    pub fn empty() -> StatementBlock {
        StatementBlock{ statements: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Declaration(DeclarationStatement),
    Assignment(AssignmentStatement),
    Expression(Expression),
    Return(ReturnStatement),

    Conditional(ConditionalStatement),
    Loop(LoopStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclarationStatement {
    pub name: Identifier,
    pub var_type: Identifier,
    pub value: Expression,
    pub is_mutable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentStatement {
    pub reference: Reference,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionalStatement {
    pub condition: Expression,
    pub body: Box<StatementBlock>,
    pub else_body: Option<Box<StatementBlock>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement {
    pub condition: Expression,
    pub body: Box<StatementBlock>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    TernaryCondition {
        condition: Box<Expression>,
        true_value: Box<Expression>,
        false_value: Box<Expression>,
    },
    BinaryOperation {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    UnaryOperation {
        operator: Operator,
        operand: Box<Expression>,
    },
    Atomic(AtomicExpression),
}

#[derive(Debug, Clone, PartialEq, )]
pub enum AtomicExpression {
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall(FunctionCallExpression),
    Parenthesized(ParenthesizedExpression),
    ArrayLiteral(ArrayLiteralExpression),
    ArrayIndex(ArrayIndexExpression),
}


#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallExpression {
    pub name: Identifier,
    pub parameters: Vec<Expression>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesizedExpression {
    pub value: Box<Expression>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteralExpression {
    pub values: Vec<Expression>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ArrayIndexExpression {
    pub array: Box<AtomicExpression>,
    pub index: ArrayIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayIndex {
    Single(Box<Expression>),
    Slice {
        start: Option<Box<Expression>>,
        end: Option<Box<Expression>>,
    },
}


#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    Identifier(Identifier),
    ArrayReference{
        array: Box<Reference>,
        index: ArrayIndex,
    },
}
