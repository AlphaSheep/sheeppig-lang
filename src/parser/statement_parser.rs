use core::panic;
use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::{ Identifier, Literal, Operator, Keyword };
use crate::tokens::Token;
use crate::tree::{
    StatementBlock, Statement,
    Expression, AtomicExpression, AssignmentStatement, Reference, DeclarationStatement,
};

use crate::parser::utils::{ handle_parse_error, handle_parse_error_for_option, handle_expression_parse_error };
use crate::parser::expression_parser::parse_expression;


pub fn parse_statement_block(tokens: &mut Peekable<Iter<Token>>) -> StatementBlock {
    if tokens.next() != Some(&Token::OpenBrace) {
        handle_parse_error_for_option::<()>("Expected open brace after function signature, found {:?}", tokens.peek());
    }

    let mut statements = vec![];

    while let Some(token) = tokens.peek() {
        match token {
            Token::Newline => { tokens.next(); },

            Token::CloseBrace => { tokens.next(); break },

            // TODO: If statement

            // TODO: While statement

            _ => statements.push(parse_statement(tokens)),
        }
    }

    StatementBlock {
        statements,
    }
}


pub fn parse_statement(all_tokens: &mut Peekable<Iter<Token>>) -> Statement {
    let tokens_vec = consume_statement_tokens(all_tokens);
    let tokens = &mut tokens_vec.iter().peekable();

    let is_variable = match tokens.peek() {
        Some(Token::Keyword(Keyword::Variable)) => {
            tokens.next();
            true
        },
        _ => false,
    };

    let mut left = parse_expression(tokens);

    let token = tokens.peek();
    match token {

        // TODO: Variable declaration

        Some(Token::Colon) => {
            tokens.next();
            parse_declaration_statement(left, tokens, is_variable)
        }

        Some(Token::Assign) => if is_variable {
            handle_parse_error_for_option("A variable declaration must be followed by a type", token)
        } else {
            tokens.next();
            let right = parse_expression(tokens);
            convert_assignment_statement(left, right)
        },

        Some(Token::BinaryAssign(operator)) => {
            tokens.next();
            let right = get_binary_expansion(left.clone(), operator, parse_expression(tokens));
            convert_assignment_statement(left, right)
        },

        None => Statement::Expression(left),

        _ => {
            handle_parse_error_for_option("Unrecognised token in statement", token)
        },
    }
}


fn consume_statement_tokens(tokens: &mut Peekable<Iter<Token>>) -> Vec<Token> {
    let mut statement_tokens = vec![];

    while let Some(token) = tokens.next() {
        match token {
            Token::Newline => break,
            _ => statement_tokens.push(token.clone()),
        }
    }

    statement_tokens
}


fn convert_assignment_statement(left: Expression, right: Expression) -> Statement {
    Statement::Assignment(
        AssignmentStatement {
            reference: convert_expression_to_reference(left),
            value: right,
        }
    )
}


fn convert_expression_to_reference(expression: Expression) -> Reference {
    match expression {
        Expression::Atomic(AtomicExpression::Identifier(identifier)) => Reference::Identifier(identifier),

        // TODO: Array index

        _ => handle_expression_parse_error("Expected a reference before an assignment.", &expression)
    }
}


fn get_binary_expansion(left: Expression, operator: &Operator, right: Expression) -> Expression {
    Expression::BinaryOperation {
        left: Box::new(left),
        operator: operator.clone(),
        right: Box::new(right),
    }
}


fn parse_declaration_statement(left: Expression, tokens: &mut Peekable<Iter<Token>>, is_variable: bool) -> Statement {
    let name = match left {
        Expression::Atomic(AtomicExpression::Identifier(identifier)) => identifier.clone(),
        _ => handle_expression_parse_error("Expected an identifier in a declaration statement", &left),
    };

    let var_type = match tokens.next() {
        Some(Token::Identifier(identifier)) => identifier.clone(),
        token => handle_parse_error_for_option("Expected a type after colon", token),
    };

    let value = match tokens.next() {
        Some(Token::Assign) => parse_expression(tokens),
        _ => handle_parse_error_for_option("Expected variable to be initialised", tokens.peek()),
    };

    Statement::Declaration(DeclarationStatement {
        name,
        var_type,
        value,
        is_mutable: is_variable,
    })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_statement_block() {
        let tokens = vec![
            Token::OpenBrace,
            Token::CloseBrace,
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_statement_block(&mut tokens);

        let expected = StatementBlock {
            statements: vec![],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_statement_block_newline() {
        let tokens = vec![
            Token::OpenBrace,
            Token::Newline,
            Token::CloseBrace,
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_statement_block(&mut tokens);

        let expected = StatementBlock {
            statements: vec![],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_statement_block_single_statement() {
        let tokens = vec![
            Token::OpenBrace,
            Token::Identifier(Identifier::Simple("identifier".to_string())),
            Token::Assign,
            Token::Literal(Literal::Integer(1)),
            Token::Newline,
            Token::CloseBrace,
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_statement_block(&mut tokens);

        let expected = StatementBlock {
            statements: vec![
                Statement::Assignment(
                    AssignmentStatement {
                        reference: Reference::Identifier(Identifier::Simple("identifier".to_string())),
                        value: Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1))),
                    }
                )
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_statement_block_two_statements() {
        let tokens = vec![
            Token::OpenBrace,
            Token::Identifier(Identifier::Simple("first".to_string())),
            Token::Assign,
            Token::Literal(Literal::Integer(1)),
            Token::Newline,
            Token::Identifier(Identifier::Simple("second".to_string())),
            Token::Assign,
            Token::Literal(Literal::Integer(2)),
            Token::Newline,
            Token::CloseBrace,
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_statement_block(&mut tokens);

        let expected = StatementBlock {
            statements: vec![
                Statement::Assignment(
                    AssignmentStatement {
                        reference: Reference::Identifier(Identifier::Simple("first".to_string())),
                        value: Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1))),
                    }
                ),
                Statement::Assignment(
                    AssignmentStatement {
                        reference: Reference::Identifier(Identifier::Simple("second".to_string())),
                        value: Expression::Atomic(AtomicExpression::Literal(Literal::Integer(2))),
                    }
                ),
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_assignment_statement() {
        let tokens = vec![
            Token::Identifier(Identifier::Simple("identifier".to_string())),
            Token::Assign,
            Token::Literal(Literal::Integer(1)),
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_statement(&mut tokens);

        let expected = Statement::Assignment(
            AssignmentStatement {
                reference: Reference::Identifier(Identifier::Simple("identifier".to_string())),
                value: Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1))),
            }
        );

        assert_eq!(result, expected)
    }

    #[test]
    fn test_parse_binary_assignment_statement() {
        let tokens = vec![
            Token::Identifier(Identifier::Simple("identifier".to_string())),
            Token::BinaryAssign(Operator::Plus),
            Token::Literal(Literal::Integer(1)),
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_statement(&mut tokens);

        let expected = Statement::Assignment(
            AssignmentStatement {
                reference: Reference::Identifier(Identifier::Simple("identifier".to_string())),
                value: Expression::BinaryOperation {
                    left: Box::new(Expression::Atomic(AtomicExpression::Identifier(Identifier::Simple("identifier".to_string())))),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))),
                },
            }
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_standalone_expression_statement() {
        let tokens = vec![
            Token::Literal(Literal::Integer(1)),
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_statement(&mut tokens);

        let expected = Statement::Expression(
            Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_consume_statement_tokens() {
        let tokens_vec = vec![
            Token::Identifier(Identifier::Simple("first".to_string())),
            Token::Assign,
            Token::Literal(Literal::Integer(1)),
            Token::Newline,
            Token::Identifier(Identifier::Simple("second".to_string())),
        ];

        let mut tokens = tokens_vec.iter().peekable();

        let result = consume_statement_tokens(&mut tokens);

        let expected = vec![
            Token::Identifier(Identifier::Simple("first".to_string())),
            Token::Assign,
            Token::Literal(Literal::Integer(1)),
        ];

        assert_eq!(result, expected);
        assert_eq!(tokens.next(), Some(&Token::Identifier(Identifier::Simple("second".to_string()))));
    }

    #[test]
    fn test_convert_assignment_statement() {
        let left = Expression::Atomic(AtomicExpression::Identifier(Identifier::Simple("identifier".to_string())));
        let right = Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)));

        let result = convert_assignment_statement(left, right);

        let expected = Statement::Assignment(
            AssignmentStatement {
                reference: Reference::Identifier(Identifier::Simple("identifier".to_string())),
                value: Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1))),
            }
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_binary_expression() {
        let left = Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)));
        let operator = Operator::Plus;
        let right = Expression::Atomic(AtomicExpression::Literal(Literal::Integer(2)));

        let result = get_binary_expansion(left, &operator, right);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))),
            operator: Operator::Plus,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(2)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_variable_declaration() {
        let left = Expression::Atomic(AtomicExpression::Identifier(Identifier::Simple("identifier".to_string())));
        let tokens = vec![
            Token::Identifier(Identifier::Simple("type".to_string())),
            Token::Assign,
            Token::Literal(Literal::Integer(1)),
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_declaration_statement(left, &mut tokens, true);

        let expected = Statement::Declaration(
            DeclarationStatement {
                name: Identifier::Simple("identifier".to_string()),
                var_type: Identifier::Simple("type".to_string()),
                value: Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1))),
                is_mutable: true,
            }
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_immutable_variable_declaration() {
        let left = Expression::Atomic(AtomicExpression::Identifier(Identifier::Simple("identifier".to_string())));
        let tokens = vec![
            Token::Identifier(Identifier::Simple("type".to_string())),
            Token::Assign,
            Token::Literal(Literal::Integer(1)),
        ];
        let mut tokens = tokens.iter().peekable();

        let result = parse_declaration_statement(left, &mut tokens, false);

        let expected = Statement::Declaration(
            DeclarationStatement {
                name: Identifier::Simple("identifier".to_string()),
                var_type: Identifier::Simple("type".to_string()),
                value: Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1))),
                is_mutable: false,
            }
        );

        assert_eq!(result, expected);
    }

}