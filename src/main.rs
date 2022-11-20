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

#[derive(PartialEq, Clone)]
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

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Operator::And => todo!(),
            Operator::Or => todo!(),
            Operator::Not => todo!(),
            Operator::Equality => todo!(),
            Operator::Comparison => write!(f, "="),
            Operator::In => todo!(),
            Operator::Match => todo!(),
            Operator::Asc => todo!(),
            Operator::Desc => todo!(),
            Operator::UnaryPlus => todo!(),
            Operator::UnaryMinus => todo!(),
            Operator::Plus => todo!(),
            Operator::Minus => todo!(),
            Operator::Star => todo!(),
            Operator::Slash => todo!(),
            Operator::Percent => todo!(),
            Operator::StarStar => todo!(),
        }
    }
}

fn print(tree: Vec<Literal>) -> String {
    let mut output = String::from("");

    for leaf in tree {
        match leaf {
            Literal::Comment(c) => output.push_str(&format!("// {}", c.trim())),
            Literal::BinaryOp(b) => {
                output.push_str(&format!("{} {:?} {}", b.lhs, b.operator, b.rhs))
            }
            _ => {}
        }
    }

    output
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

    let binary_op = choice::<_, Simple<char>>((
        just::<_, _, Simple<char>>("==").to(Operator::Equality),
        just::<_, _, Simple<char>>("=").to(Operator::Comparison),
        just::<_, _, Simple<char>>("&&").to(Operator::And),
    ));

    // let unary_op = choice::<_, Simple<char>>((just::<_, _, Simple<char>>("!").to(Operator::Not), ));
    let unary_op = just("!").to(Operator::Not);

    let binary_op = ident::<_, Simple<char>>()
        .then(binary_op.padded())
        .then(ident())
        .map(|((lhs, operator), rhs)| Literal::BinaryOp(Binary { operator, lhs, rhs }));

    let unary_op = unary_op
        .then(ident())
        .map(|(operator, rhs)| Literal::UnaryOp(Unary { operator, rhs }));

    let result = choice::<_, Simple<char>>((comment, bool, binary_op, unary_op))
        .repeated()
        .padded();

    result.parse(input)
}

fn main() {
    let test = r#"something=som false true // single line comment
"#;

    println!("{:?}", ast(test));
    println!("{:?}", print(ast(test).unwrap()))
}
