use std::fmt;

use chumsky::{combinator::Map, error::Cheap, prelude::*, primitive::Filter, text::ident};

#[derive(Debug, PartialEq, Clone)]
enum Literal {
    Null,
    Bool(bool),
    Number,
    String,
    Array,
    BinaryOp(Binary<String>),
    UnaryOp(Unary<String>),
    Object,
    Comment(String),
}

// struct Comment(String);

#[derive(Debug, PartialEq, Clone)]
enum SimpleExpression {
    This,
    ThisAttribute,
    Everything,
    Parent,
    FuncCall,
}

#[derive(Debug, PartialEq, Clone)]
enum CompoundExpression {
    Parenthesis,
    TraversalExpression,
    PipeFuncCall,
}

#[derive(PartialEq, Debug, Clone)]
enum Operator {
    And,
    Or,
    Not,
    Equality,
    Comparison,
    In,
    Match,
    Asc,
    Desc,
    UnaryPlus,
    UnaryMinus,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
}

#[derive(PartialEq, Debug, Clone)]
struct Unary<T> {
    operator: Operator,
    rhs: T,
}

#[derive(PartialEq, Debug, Clone)]
struct Binary<T> {
    operator: Operator,
    lhs: T,
    rhs: T,
}

fn ast(input: &str) -> Result<Vec<Literal>, Vec<Simple<char>>> {
    let comment = just("//").then(take_until(text::newline())).map(|c| {
        let (value, _) = c.1;

        Literal::Comment(value.into_iter().collect())
    });

    let bool = text::keyword("true")
        .padded()
        .to(Literal::Bool(true))
        .or(text::keyword("false").padded().to(Literal::Bool(false)));

    let equality = ident::<_, Simple<char>>()
        .then(just("==").padded())
        .then(ident())
        .map(|((lhs, _), rhs)| {
            Literal::BinaryOp(Binary {
                operator: Operator::Equality,
                lhs,
                rhs,
            })
        });

    let result = choice::<_, Simple<char>>((comment, bool, equality))
        .repeated()
        .padded();

    result.parse(input)
}

fn main() {
    let test = r#"something == som false true // single line comment
"#;

    println!("{:?}", ast(test));
    println!("{}", test.trim())
}
