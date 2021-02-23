pub mod eval;
pub mod parse;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Sqr(Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

#[cfg(test)]
mod tests {
    use crate::{
        parse::parse,
        eval::eval,
    };

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
