use std::fmt;

use chumsky::{combinator::Map, error::Cheap, prelude::*, primitive::Filter};

// string is temporary, these need to be typed
enum Expression {
    Literal(Literal),
    SimpleExpression(SimpleExpression),
    CompoundExpression(CompoundExpression),
    OperatorCall(OperatorCall),
    Comment(Unary<CommentType, String>),
}

#[derive(Debug)]
enum Literal {
    Null,
    Boolean,
    Number,
    String,
    Array,
    Object,
}

#[derive(Debug)]
enum SimpleExpression {
    This,
    ThisAttribute,
    Everything,
    Parent,
    FuncCall,
}

#[derive(Debug)]
enum CompoundExpression {
    Parenthesis,
    TraversalExpression,
    PipeFuncCall,
}

enum OperatorCall {
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
    Comment,
}

#[derive(Debug)]
enum CommentType {
    Inline,
    Single,
}

struct Unary<T, U> {
    operand: OperatorCall,
    unary_type: T,
    value: U,
}

impl fmt::Debug for OperatorCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OperatorCall::And => todo!(),
            OperatorCall::Or => todo!(),
            OperatorCall::Not => todo!(),
            OperatorCall::Equality => todo!(),
            OperatorCall::Comparison => todo!(),
            OperatorCall::In => todo!(),
            OperatorCall::Match => todo!(),
            OperatorCall::Asc => todo!(),
            OperatorCall::Desc => todo!(),
            OperatorCall::UnaryPlus => todo!(),
            OperatorCall::UnaryMinus => todo!(),
            OperatorCall::Plus => todo!(),
            OperatorCall::Minus => todo!(),
            OperatorCall::Star => todo!(),
            OperatorCall::Slash => todo!(),
            OperatorCall::Percent => todo!(),
            OperatorCall::StarStar => todo!(),
            OperatorCall::Comment => write!(f, "//"),
        }
    }
}

impl fmt::Debug for Unary<CommentType, String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.unary_type {
            CommentType::Inline => write!(f, "{:?} {}", &self.operand, &self.value),
            CommentType::Single => {
                if &self.value.len() > &10 {
                    write!(f, "this will be multiline comment")
                } else {
                    write!(f, "{:?} {}", &self.operand, &self.value)
                }
            }
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Expression::Literal(_) => todo!(),
            Expression::SimpleExpression(_) => todo!(),
            Expression::CompoundExpression(_) => todo!(),
            Expression::OperatorCall(_) => todo!(),
            Expression::Comment(c) => write!(f, "{:?}", c),
        }
    }
}

fn main() {
    let word = filter::<_, _, Cheap<char>>(|c: &char| c.is_alphanumeric() || c.is_whitespace())
        .repeated()
        .at_least(1);

    let inline_comment =
        filter::<_, _, Cheap<char>>(|c: &char| c.is_alphanumeric() || c.is_whitespace())
            .repeated()
            .at_least(1)
            .ignored()
            .then(just('/'))
            .then(just('/'))
            .then(just(' '))
            .then(word)
            .map(|c| {
                Expression::Comment(Unary {
                    unary_type: CommentType::Inline,
                    value: c.1.into_iter().collect::<String>(),
                    operand: OperatorCall::Comment,
                })
            });

    let single_line_comment = just::<_, _, Cheap<char>>('/')
        .then(just('/'))
        .then(just(' '))
        .then(word)
        .map(|c| {
            Expression::Comment(Unary {
                unary_type: CommentType::Single,
                value: c.1.into_iter().collect::<String>(),
                operand: OperatorCall::Comment,
            })
        });

    let groq = "this // this is a comment";

    match inline_comment.parse(groq) {
        Ok(result) => println!("{:?}", result),
        Err(_) => println!("not matched!"),
    }

    match single_line_comment.parse(groq) {
        Ok(result) => println!("{:?}", result),
        Err(_) => println!("not matched!"),
    }
}
