
use crate::elements::{Identifier, Literal, Operator};

#[derive(Debug, PartialEq)]
pub struct Module {
    pub name: Identifier,
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
}

#[derive(Debug, PartialEq)]
pub struct Import {
    pub name: Identifier,
    pub alias: Identifier,
    pub source: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Identifier>,
    pub body: Box<StatementBlock>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub param_type: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct StatementBlock {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    VarDeclaration(VarDeclarationStatement),
    ConstDeclaration(ConstDeclarationStatement),
    Assignment(AssignmentStatement),
    Expression(Expression),
    Return(ReturnStatement),

    Conditional(ConditionalStatement),
    Loop(LoopStatement),
}

#[derive(Debug, PartialEq)]
pub struct VarDeclarationStatement {
    pub name: Identifier,
    pub var_type: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ConstDeclarationStatement {
    pub name: Identifier,
    pub const_type: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct AssignmentStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ConditionalStatement {
    pub condition: Expression,
    pub body: Box<StatementBlock>,
    pub else_body: Option<Box<StatementBlock>>,
}

#[derive(Debug, PartialEq)]
pub struct LoopStatement {
    pub condition: Expression,
    pub body: Box<StatementBlock>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum AtomicExpression {
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall(FunctionCallExpression),
    Parenthesized(ParenthesizedExpression),
}

#[derive(Debug, PartialEq)]
pub struct FunctionCallExpression {
    pub name: Identifier,
    pub parameters: Vec<Expression>,
}



#[derive(Debug, PartialEq)]
pub struct ParenthesizedExpression {
    pub value: Box<Expression>,
}
