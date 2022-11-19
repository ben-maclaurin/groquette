use std::fmt;

use chumsky::{combinator::Map, error::Cheap, prelude::*, primitive::Filter};

// string is temporary, these need to be typed
#[derive(PartialEq)]
enum Expression {
    Literal(Literal),
    SimpleExpression(SimpleExpression),
    CompoundExpression(CompoundExpression),
    OperatorCall(OperatorCall),
    Comment(Unary<CommentType, String>),
}

#[derive(Debug, PartialEq)]
enum Literal {
    Null,
    Boolean,
    Number,
    String,
    Array,
    Object,
}

#[derive(Debug, PartialEq)]
enum SimpleExpression {
    This,
    ThisAttribute,
    Everything,
    Parent,
    FuncCall,
}

#[derive(Debug, PartialEq)]
enum CompoundExpression {
    Parenthesis,
    TraversalExpression,
    PipeFuncCall,
}

#[derive(PartialEq)]
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

#[derive(Debug, PartialEq)]
enum CommentType {
    Inline,
    Single,
}

#[derive(PartialEq)]
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
            CommentType::Inline => write!(f, "{:?}{}", &self.operand, &self.value),
            CommentType::Single => {
                if &self.value.len() > &10 {
                    write!(f, "{:?}{}", &self.operand, &self.value)
                } else {
                    write!(f, "{:?}{}", &self.operand, &self.value)
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

fn parse_comment(input: &str) -> Result<Expression, Vec<Cheap<char>>> {
    let word = any::<_, Cheap<char>>().repeated().at_least(1);

    let inline_comment = take_until(just("//"))
        .ignored()
        .then(take_until(text::newline()))
        .map(|c| {
            let (value, _) = c.1;

            Expression::Comment(Unary {
                unary_type: CommentType::Inline,
                value: value.into_iter().collect(),
                operand: OperatorCall::Comment,
            })
        });

    let single_line_comment = just::<_, _, Cheap<char>>("//")
        .then(take_until(text::newline()))
        .map(|c| {
            let (value, _) = c.1;

            Expression::Comment(Unary {
                unary_type: CommentType::Single,
                value: value.into_iter().collect(),
                operand: OperatorCall::Comment,
            })
        });

    let result = inline_comment.or(single_line_comment);

    result.parse(input)
}

fn ast(input: &str) -> Vec<Expression> {
    return vec![parse_comment(input).unwrap()];
}

fn main() {
    let single_line_comment = r#"// this is a single line comment
"#;

    println!("{:?}", ast(single_line_comment));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_parse_single_line_comment() {
        let single_line_comment = "// single line comment";

        assert_eq!(
            parse_comment(single_line_comment).unwrap(),
            Expression::Comment(Unary {
                operand: OperatorCall::Comment,
                unary_type: CommentType::Single,
                value: "single line comment".to_string()
            })
        );
    }

    #[test]
    fn can_parse_inline_comment() {
        let inline_comment = "text // inline comment";

        assert_eq!(
            parse_comment(inline_comment).unwrap(),
            Expression::Comment(Unary {
                operand: OperatorCall::Comment,
                unary_type: CommentType::Inline,
                value: "inline comment".to_string()
            })
        );
    }
}
