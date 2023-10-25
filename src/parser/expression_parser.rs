use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::Operator;
use crate::tokens::Token;
use crate::tree::Expression;

use crate::parser::utils::{handle_parse_error, handle_parse_error_for_option};
use crate::parser::atomic_parser::parse_atomic;


const NUM_PRECEDENCE_LEVELS: usize = 12;
const PRECEDENCE_TABLE: [&[Operator]; NUM_PRECEDENCE_LEVELS] = [
    &[Operator::Power],
    &[], // Unary operators
    &[Operator::Times, Operator::Divide, Operator::Modulo],
    &[Operator::Plus, Operator::Minus],
    &[Operator::BitwiseLeftShift, Operator::BitwiseRightShift],
    &[Operator::LessThan, Operator::LessThanOrEqual, Operator::GreaterThan, Operator::GreaterThanOrEqual],
    &[Operator::Equal, Operator::NotEqual],
    &[Operator::BitwiseAnd],
    &[Operator::BitwiseXor],
    &[Operator::BitwiseOr],
    &[Operator::And],
    &[Operator::Or],
];


pub fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    // let left = parse_logical_or(tokens);
    let left = parse_binary_expression_with_precedence(tokens, NUM_PRECEDENCE_LEVELS-1);

    match tokens.peek() {
        Some(Token::TernaryCondition) => {
            tokens.next();
            let true_value = parse_expression(tokens);
            match tokens.next() {
                Some(Token::Colon) => {},
                _ => handle_parse_error_for_option("Expected colon after ternary condition", tokens.peek()),
            }
            let false_value = parse_expression(tokens);
            Expression::TernaryCondition {
                condition: Box::new(left),
                true_value: Box::new(true_value),
                false_value: Box::new(false_value),
            }
        },
        _ => left,
    }
}


fn parse_binary_expression_with_precedence(tokens: &mut Peekable<Iter<Token>>, precedence: usize) -> Expression {
    if precedence >= NUM_PRECEDENCE_LEVELS {
        panic!("Invalid precedence level: {}", precedence)
    }

    let operators = PRECEDENCE_TABLE[precedence];

    match precedence {
        0 => parse_binary_operation(tokens,
            |tokens| parse_atomic(tokens),
            |tokens| parse_binary_expression_with_precedence(tokens, precedence),
            operators
        ),

        1 => parse_unary(tokens),

        _ => parse_binary_operation(tokens,
            |tokens| parse_binary_expression_with_precedence(tokens, precedence - 1),
            |tokens| parse_binary_expression_with_precedence(tokens, precedence),
            operators
        )
    }
}


fn parse_binary_operation<F, G>(
    tokens: &mut Peekable<Iter<Token>>,
    parse_left: F,
    parse_right: G,
    operators: &[Operator],
) -> Expression
where
    F: Fn(&mut Peekable<Iter<Token>>) -> Expression,
    G: Fn(&mut Peekable<Iter<Token>>) -> Expression,
{
    let left = parse_left(tokens);
    match tokens.peek() {
        Some(Token::Operator(operator)) => {
            if operators.contains(operator) {
                tokens.next();
                Expression::BinaryOperation {
                    left: Box::new(left),
                    operator: operator.clone(),
                    right: Box::new(parse_right(tokens)),
                }
            } else {
                left
            }
        }
        _ => left,
    }
}


fn parse_unary(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    match tokens.peek() {
        Some(token @ Token::Operator(operator)) => match operator {
            Operator::Plus | Operator::Minus | Operator::Not | Operator::BitwiseNot => {
                tokens.next();
                Expression::UnaryOperation {
                    operator: operator.clone(),
                    operand: Box::new(parse_unary(tokens)),
                }
            }
            _ => handle_parse_error("Operator not allowed in unary expression", token),
        },
        _ => parse_binary_expression_with_precedence(tokens, 0),
    }
}



#[cfg(test)]
mod test {
    use crate::elements::Literal;
    use crate::tree::AtomicExpression;

    use super::*;

    #[test]
    fn test_parse_power() {
        let tokens = vec![
            Token::Literal(Literal::Integer(1)),
            Token::Operator(Operator::Power),
            Token::Literal(Literal::Integer(2)),
            Token::Operator(Operator::Plus),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 0);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))),
            operator: Operator::Power,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(2)))),
        };

        assert_eq!(result, expected);
        assert_eq!(Token::Operator(Operator::Plus), *tokens.next().unwrap());
    }

    #[test]
    fn test_parse_unary() {
        let tokens = vec![
            Token::Operator(Operator::Minus),
            Token::Literal(Literal::Integer(1)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(2)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 1);

        let expected = Expression::UnaryOperation {
            operator: Operator::Minus,
            operand: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))),
        };

        assert_eq!(result, expected);
        assert_eq!(Token::Operator(Operator::Plus), *tokens.next().unwrap());
    }

    #[test]
    fn test_parse_unary_pass_through() {
        let tokens = vec![
            Token::Literal(Literal::Integer(1)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(2)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 1);

        let expected = Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)));

        assert_eq!(result, expected);
        assert_eq!(Token::Operator(Operator::Plus), *tokens.next().unwrap());
    }

    #[test]
    fn test_unary_power_precedence() {
        let tokens = vec![
            Token::Operator(Operator::Minus),
            Token::Literal(Literal::Integer(1)),
            Token::Operator(Operator::Power),
            Token::Literal(Literal::Integer(2)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 1);

        let expected = Expression::UnaryOperation {
            operator: Operator::Minus,
            operand: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))),
                operator: Operator::Power,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(2)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_factors() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 2);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Times,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_factor_power_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Power),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 2);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Times,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::Power,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_factor_power_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Power),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 2);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::Power,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::Times,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_sums() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 3);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Plus,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_sum_factors_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 3);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Plus,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::Times,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_sum_factors_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 3);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::Times,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::Plus,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_shift() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseLeftShift),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 4);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseLeftShift,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_shift_sum_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseLeftShift),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 4);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseLeftShift,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::Plus,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_shift_sum_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::BitwiseLeftShift),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 4);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::Plus,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::BitwiseLeftShift,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }


    #[test]
    fn test_parse_relation() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 5);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::LessThan,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_relation_shift_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::BitwiseLeftShift),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 5);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::LessThan,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::BitwiseLeftShift,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_relation_shift_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseLeftShift),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 5);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::BitwiseLeftShift,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::LessThan,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_equality() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Equal),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 6);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Equal,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equality_relation_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Equal),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 6);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Equal,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::LessThan,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equality_relation_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Equal),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 6);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::LessThan,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::Equal,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_bitwise_and() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseAnd),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 7);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseAnd,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitwise_and_equality_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseAnd),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Equal),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 7);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseAnd,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::Equal,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitwise_and_equality_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Equal),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::BitwiseAnd),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 7);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::Equal,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::BitwiseAnd,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_bitwise_xor() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseXor),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 8);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseXor,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitwise_xor_bitwise_and_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseXor),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::BitwiseAnd),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 8);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseXor,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::BitwiseAnd,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitwise_xor_bitwise_and_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseAnd),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::BitwiseXor),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 8);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::BitwiseAnd,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::BitwiseXor,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_bitwise_or() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseOr),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 9);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseOr,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitwise_or_bitwise_xor_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseOr),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::BitwiseXor),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 9);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseOr,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::BitwiseXor,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitwise_or_bitwise_xor_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseXor),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::BitwiseOr),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 9);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::BitwiseXor,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::BitwiseOr,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_logical_and() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::And),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 10);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::And,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_logical_and_bitwise_or_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::And),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::BitwiseOr),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 10);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::And,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::BitwiseOr,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_logical_and_bitwise_or_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseOr),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::And),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 10);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::BitwiseOr,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::And,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_logical_or() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Or),
            Token::Literal(Literal::Integer(4)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 11);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Or,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_logical_or_logical_and_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Or),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::And),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 11);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Or,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::And,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_logical_or_logical_and_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::And),
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Or),
            Token::Literal(Literal::Integer(5)),
        ];
        let tokens = &mut tokens.iter().peekable();
        let result = parse_binary_expression_with_precedence(tokens, 11);

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::And,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::Or,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_expression_with_ternary() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::TernaryCondition,
            Token::Literal(Literal::Integer(4)),
            Token::Colon,
            Token::Literal(Literal::Integer(5)),
        ];

        let expected = Expression::TernaryCondition {
            condition: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            true_value: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            false_value: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_expression(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_ternary_logical_or_left_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Or),
            Token::Literal(Literal::Integer(4)),
            Token::TernaryCondition,
            Token::Literal(Literal::Integer(5)),
            Token::Colon,
            Token::Literal(Literal::Integer(6)),
        ];

        let expected = Expression::TernaryCondition {
            condition:
                Box::new(Expression::BinaryOperation {
                    left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                    operator: Operator::Or,
                    right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                }),
            true_value: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            false_value: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(6)))),
        };

        assert_eq!(parse_expression(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_ternary_logical_or_middle_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::TernaryCondition,
            Token::Literal(Literal::Integer(4)),
            Token::Operator(Operator::Or),
            Token::Literal(Literal::Integer(5)),
            Token::Colon,
            Token::Literal(Literal::Integer(6)),
        ];

        let expected = Expression::TernaryCondition {
            condition: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            true_value:
                Box::new(Expression::BinaryOperation {
                    left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                    operator: Operator::Or,
                    right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
                }),
            false_value: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(6)))),
        };

        assert_eq!(parse_expression(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_ternary_logical_or_right_precedence() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::TernaryCondition,
            Token::Literal(Literal::Integer(4)),
            Token::Colon,
            Token::Literal(Literal::Integer(5)),
            Token::Operator(Operator::Or),
            Token::Literal(Literal::Integer(6)),
        ];

        let expected = Expression::TernaryCondition {
            condition: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            true_value: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            false_value:
                Box::new(Expression::BinaryOperation {
                    left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
                    operator: Operator::Or,
                    right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(6)))),
                }),
        };

        assert_eq!(parse_expression(&mut tokens.iter().peekable()), expected);
    }

}
