use core::panic;
use std::iter::Peekable;
use std::slice::Iter;

use crate::elements::{Identifier, Literal, Operator, Keyword};
use crate::tokens::Token;
use crate::tree::{
    StatementBlock, Statement,
    Expression, AtomicExpression, ParenthesizedExpression, FunctionCallExpression
};

use crate::parser::utils::{handle_parse_error, handle_parse_error_for_option};
use crate::parser::atomic_parser::parse_atomic;


pub fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    let left = parse_logical_or(tokens);
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


fn parse_binary_operation(
    tokens: &mut Peekable<Iter<Token>>,
    parse_left: fn(&mut Peekable<Iter<Token>>) -> Expression,
    parse_right: fn(&mut Peekable<Iter<Token>>) -> Expression,
    operators: &[Operator],
) -> Expression {
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


fn parse_logical_or(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_logical_and,
        parse_logical_or,
        &[Operator::Or]
    )
}


fn parse_logical_and(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_bitwise_or,
        parse_logical_and,
        &[Operator::And]
    )
}

fn parse_bitwise_or(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_bitwise_xor,
        parse_bitwise_or,
        &[Operator::BitwiseOr]
    )
}


fn parse_bitwise_xor(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_bitwise_and,
        parse_bitwise_xor,
        &[Operator::BitwiseXor]
    )
}


fn parse_bitwise_and(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_equality,
        parse_bitwise_and,
        &[Operator::BitwiseAnd]
    )
}


fn parse_equality(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_relation,
        parse_equality,
        &[Operator::Equal, Operator::NotEqual]
    )
}


fn parse_relation(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_shift,
        parse_relation,
        &[Operator::LessThan, Operator::LessThanOrEqual, Operator::GreaterThan, Operator::GreaterThanOrEqual]
    )
}


fn parse_shift(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_sum,
        parse_shift,
        &[Operator::BitwiseLeftShift, Operator::BitwiseRightShift]
    )
}


fn parse_sum(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_factor,
        parse_sum,
        &[Operator::Plus, Operator::Minus]
    )
}


fn parse_factor(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_power,
        parse_factor,
        &[Operator::Times, Operator::Divide, Operator::Modulo]
    )
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
        _ => parse_power(tokens),
    }
}


fn parse_power(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    parse_binary_operation(tokens,
        parse_atomic,
        parse_power,
        &[Operator::Power]
    )
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_power() {
        let tokens = vec![
            Token::Literal(Literal::Integer(1)),
            Token::Operator(Operator::Power),
            Token::Literal(Literal::Integer(2)),
            Token::Operator(Operator::Plus),
        ];
        let iter_tokens = &mut tokens.iter().peekable();

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))),
            operator: Operator::Power,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(2)))),
        };

        assert_eq!(parse_power(iter_tokens), expected);
        assert_eq!(Token::Operator(Operator::Plus), *iter_tokens.next().unwrap());
    }

    #[test]
    fn test_parse_unary() {
        let tokens = vec![
            Token::Operator(Operator::Minus),
            Token::Literal(Literal::Integer(1)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(2)),
        ];
        let iter_tokens = &mut tokens.iter().peekable();

        let expected = Expression::UnaryOperation {
            operator: Operator::Minus,
            operand: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))),
        };

        assert_eq!(parse_unary(iter_tokens), expected);
        assert_eq!(Token::Operator(Operator::Plus), *iter_tokens.next().unwrap());
    }

    #[test]
    fn test_parse_unary_pass_through() {
        let tokens = vec![
            Token::Literal(Literal::Integer(1)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(2)),
        ];
        let iter_tokens = &mut tokens.iter().peekable();

        let expected = Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)));

        assert_eq!(parse_unary(iter_tokens), expected);
        assert_eq!(Token::Operator(Operator::Plus), *iter_tokens.next().unwrap());
    }

    #[test]
    fn test_unary_power_precedence() {
        let tokens = vec![
            Token::Operator(Operator::Minus),
            Token::Literal(Literal::Integer(1)),
            Token::Operator(Operator::Power),
            Token::Literal(Literal::Integer(2)),
        ];

        let expected = Expression::UnaryOperation {
            operator: Operator::Minus,
            operand: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(1)))),
                operator: Operator::Power,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(2)))),
            }),
        };

        assert_eq!(parse_unary(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_factors() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Times),
            Token::Literal(Literal::Integer(4)),

        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Times,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_factor(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Times,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::Power,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_factor(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::Power,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::Times,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_factor(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_sums() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Plus),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Plus,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_sum(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Plus,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::Times,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_sum(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::Times,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::Plus,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_sum(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_shift() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseLeftShift),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseLeftShift,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_shift(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseLeftShift,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::Plus,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_shift(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::Plus,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::BitwiseLeftShift,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_shift(&mut tokens.iter().peekable()), expected);
    }


    #[test]
    fn test_parse_relation() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::LessThan),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::LessThan,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_relation(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::LessThan,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::BitwiseLeftShift,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_relation(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::BitwiseLeftShift,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::LessThan,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_relation(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_equality() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Equal),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Equal,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_equality(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Equal,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::LessThan,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_equality(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::LessThan,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::Equal,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_equality(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_bitwise_and() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseAnd),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseAnd,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_bitwise_and(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseAnd,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::Equal,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_bitwise_and(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::Equal,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::BitwiseAnd,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_bitwise_and(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_bitwise_xor() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseXor),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseXor,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_bitwise_xor(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseXor,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::BitwiseAnd,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_bitwise_xor(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::BitwiseAnd,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::BitwiseXor,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_bitwise_xor(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_bitwise_or() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::BitwiseOr),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseOr,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_bitwise_or(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::BitwiseOr,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::BitwiseXor,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_bitwise_or(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::BitwiseXor,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::BitwiseOr,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_bitwise_or(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_logical_and() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::And),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::And,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_logical_and(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::And,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::BitwiseOr,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_logical_and(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::BitwiseOr,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::And,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_logical_and(&mut tokens.iter().peekable()), expected);
    }

    #[test]
    fn test_parse_logical_or() {
        let tokens = vec![
            Token::Literal(Literal::Integer(3)),
            Token::Operator(Operator::Or),
            Token::Literal(Literal::Integer(4)),
        ];

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Or,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
        };

        assert_eq!(parse_logical_or(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
            operator: Operator::Or,
            right: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
                operator: Operator::And,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
            }),
        };

        assert_eq!(parse_logical_or(&mut tokens.iter().peekable()), expected);
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

        let expected = Expression::BinaryOperation {
            left: Box::new(Expression::BinaryOperation {
                left: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(3)))),
                operator: Operator::And,
                right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(4)))),
            }),
            operator: Operator::Or,
            right: Box::new(Expression::Atomic(AtomicExpression::Literal(Literal::Integer(5)))),
        };

        assert_eq!(parse_logical_or(&mut tokens.iter().peekable()), expected);
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