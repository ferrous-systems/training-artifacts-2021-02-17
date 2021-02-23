pub mod eval;
pub mod parse;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Sqr(Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

impl FromStr for Expr {
    type Err = parse::ParseError;

    fn from_str(the_str: &str) -> Result<Self, Self::Err> {
        parse::parse(the_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parse::{parse, ParseError},
        eval::eval,
        Expr
    };
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        let text = "3 sqr";
        let _parsed = Expr::from_str(text).unwrap();
        let _parsed2 = text.parse::<Expr>().unwrap();
        let _parsed3: Expr = text.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = r#"called `Result::unwrap()` on an `Err` value: UnexpectedInput("bad")"#)]
    fn test_from_str_bad() {
        let text = "bad";
        let _parsed = Expr::from_str(text).unwrap();
        let _parsed2 = text.parse::<Expr>().unwrap();
        let _parsed3: Expr = text.parse().unwrap();
    }

    #[test]
    fn test_from_str_bad_no_panic() {
        let text = "bad";
        let parsed = Expr::from_str(text);
        assert_eq!(parsed, Err(ParseError::UnexpectedInput("bad".to_string())))
    }

    #[test]
    fn round_trip_sqr() {
        let text = "3 sqr";
        let expected = 9i64;
        let parsed = parse(text).unwrap();
        let evald = eval(&parsed).unwrap();
        assert_eq!(expected, evald);
    }

    #[test]
    fn round_trip_add() {
        let text = "3 2 +";
        let expected = 5i64;
        let parsed = parse(text).unwrap();
        let evald = eval(&parsed).unwrap();
        assert_eq!(expected, evald);
    }

    #[test]
    fn round_trip_examples() {
        let expressions = &[
            "92",
            "40 2 +",
            "1 3 + 2 /",
            "3 sqr 4 sqr + 5 sqr -",
        ];

        let expectations = &[
            92i64,
            42,
            2,
            0
        ];

        assert_eq!(expressions.len(), expectations.len());

        // iter_a
        // iter_b
        // iter -> (a, b)

        for (expr, expected) in expressions.iter().zip(expectations) {
            let parsed = parse(expr).unwrap();
            let evald = eval(&parsed).unwrap();
            println!("'{}' => {}", expr, expected);
            assert_eq!(*expected, evald);
        }
    }

}
