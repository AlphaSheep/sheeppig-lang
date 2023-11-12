mod test_utils;

use test_utils::read_file;

use sheeppig::elements::{Identifier, Literal, Operator, Keyword};
use sheeppig::tree::{Statement, Expression, Module, Function, StatementBlock, FunctionCallExpression, AtomicExpression, DeclarationStatement};
use sheeppig::lexer::tokenize;
use sheeppig::parser::parse;


#[test]
fn test_parse_hello_world() {
    let source_code = read_file("./samples/test_samples/hello_world.sp");

    let tokens = tokenize(&source_code);
    let tree = parse(&tokens);

    let func_call = FunctionCallExpression {
        name: Identifier::Simple("print".to_string()),
        parameters: vec![
            Expression::Atomic(
                AtomicExpression::Literal(
                    Literal::String("Hello, world!".to_string())
                )
            )
        ]
    };

    let func_body = Box::new(StatementBlock {
        statements: vec![
            Statement::Expression(
                Expression::Atomic(
                    AtomicExpression::FunctionCall(func_call)
                ),
            ),
        ]
    });

    let expected = Module {
        name: Identifier::Simple("main".to_string()),
        imports: vec![],
        functions: vec![
            Function {
                name: Identifier::Simple("main".to_string()),
                parameters: vec![],
                return_type: None,
                body: func_body,
            }
        ],
        statements: StatementBlock::empty(),
    };

    assert_eq!(tree, expected);
}

#[test]
fn test_parse_comments() {
    let source_code = read_file("./samples/test_samples/comments.sp");

    let tokens = tokenize(&source_code);
    let tree = parse(&tokens);

    let func_body = Box::new(StatementBlock {
        statements: vec![
            Statement::Declaration(DeclarationStatement {
                name: Identifier::Simple("a".to_string()),
                var_type: Identifier::Simple("int".to_string()),
                value: Expression::Atomic(
                    AtomicExpression::Literal(
                        Literal::Integer(1)
                    )
                ),
                is_mutable: true,
            }),
            Statement::Declaration(DeclarationStatement {
                name: Identifier::Simple("b".to_string()),
                var_type: Identifier::Simple("int".to_string()),
                value: Expression::BinaryOperation {
                    left: Box::new(Expression::Atomic(
                        AtomicExpression::Literal(Literal::Integer(1))
                    )),
                    operator: Operator::Plus,
                    right: Box::new(Expression::Atomic(
                        AtomicExpression::Literal(Literal::Integer(2))
                    )),
                },
                is_mutable: false,
            }),
            Statement::Declaration(DeclarationStatement {
                name: Identifier::Simple("c".to_string()),
                var_type: Identifier::Simple("int".to_string()),
                value: Expression::Atomic(
                    AtomicExpression::Literal(
                        Literal::Integer(3)
                    )
                ),
                is_mutable: false,
            })
        ]
    });

    let expected = Module {
        name: Identifier::Simple("main".to_string()),
        imports: vec![],
        functions: vec![
            Function {
                name: Identifier::Simple("main".to_string()),
                parameters: vec![],
                return_type: None,
                body: func_body,
            }
        ],
        statements: StatementBlock::empty(),
    };

    assert_eq!(tree, expected);
}