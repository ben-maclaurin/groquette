use std::fmt;

use chumsky::{combinator::Map, error::Cheap, prelude::*, primitive::Filter};

// string is temporary, these need to be typed
#[derive(PartialEq, Debug)]
enum Expression {
    Literal(Literal),
    SimpleExpression(SimpleExpression),
    CompoundExpression(CompoundExpression),
    OperatorCall(OperatorCall),
    Comment(Unary<String>),
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

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
struct Unary<U> {
    operand: OperatorCall,
    value: U,
}

fn parse_comment(input: &str) -> Result<Expression, Vec<Cheap<char>>> {
    let word = any::<_, Cheap<char>>().repeated().at_least(1);

    let inline_comment = take_until(just("// "))
        .ignored()
        .then(take_until(text::newline()))
        .map(|c| {
            let (value, _) = c.1;

            Expression::Comment(Unary {
                value: value.into_iter().collect(),
                operand: OperatorCall::Comment,
            })
        });

    let result = inline_comment;

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
