use std::fmt;

use chumsky::{combinator::Map, error::Cheap, prelude::*, primitive::Filter};

// string is temporary, these need to be typed
#[derive(PartialEq, Debug, Clone)]
enum Expression {
    Literal(Literal),
    SimpleExpression(SimpleExpression),
    CompoundExpression(CompoundExpression),
    OperatorCall(OperatorCall),
    Comment(Unary<String>),
}

#[derive(Debug, PartialEq, Clone)]
enum Literal {
    Null,
    Boolean(bool),
    Number,
    String,
    Array,
    Object,
}

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

#[derive(PartialEq, Debug, Clone)]
struct Unary<U> {
    operand: OperatorCall,
    value: U,
}

fn ast(input: &str) -> Result<Vec<Expression>, Vec<Simple<char>>> {
    let comment = just("//").then(take_until(text::newline())).map(|c| {
        let (value, _) = c.1;

        Expression::Comment(Unary {
            value: value.into_iter().collect(),
            operand: OperatorCall::Comment,
        })
    });

    let result = choice::<_, Simple<char>>((
        comment,
        text::keyword("true")
            .padded()
            .to(Expression::Literal(Literal::Boolean(true))),
        text::keyword("false")
            .padded()
            .to(Expression::Literal(Literal::Boolean(false))),
    ))
    .padded()
    .repeated();

    result.parse(input)
}

fn main() {
    let test = r#"false true // single line comment
"#;

    println!("{:?}", ast(test));
    println!("{}", test.trim())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_parse_single_line_comment() {
        let single_line_comment = r#"
	    // single line comment
	"#;

        assert_eq!(
            parse_comment(single_line_comment).unwrap(),
            Expression::Comment(Unary {
                operand: OperatorCall::Comment,
                value: "single line comment".to_string()
            })
        );
    }

    #[test]
    fn can_parse_inline_comment() {
        let inline_comment = r#"
	    *[name == "Michael"] // inline comment
	"#;

        assert_eq!(
            parse_comment(inline_comment).unwrap(),
            Expression::Comment(Unary {
                operand: OperatorCall::Comment,
                value: "inline comment".to_string()
            })
        );
    }
}
