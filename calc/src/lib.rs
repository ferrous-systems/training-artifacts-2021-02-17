use std::fmt::{
    Formatter,
    Error as FmtError,
    Display
};


pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();

    for word in input.split_ascii_whitespace() {
        match word {
            "sqr" => {
                let item = match stack.pop() {
                    Some(i) => i,
                    None => return Err(ParseError::InsufficientNumbers),
                };

                let exp_sqr = Expr::Sqr(Box::new(item));
                stack.push(exp_sqr);
            }
            "+" | "-" | "*" | "/" => {
                let (a, b) = match(stack.pop(), stack.pop()) {
                    (Some(b), Some(a)) => {
                        (a, b)
                    }
                    _ => return Err(ParseError::InsufficientNumbers),
                };

                let expr = match word {

                    "+" => Expr::Plus(Box::new(a), Box::new(b)),
                    "-" => Expr::Minus(Box::new(a), Box::new(b)),
                    "*" => Expr::Multiply(Box::new(a), Box::new(b)),
                    "/" => Expr::Divide(Box::new(a), Box::new(b)),
                    whatever => {
                        // Shouldn't be possible, checked
                        return Err(ParseError::UnexpectedInput(whatever.to_string()));
                    }

                };

                stack.push(expr);
            }
            x => {
                if let Ok(num) = x.parse::<i64>() {
                    let exp_num = Expr::Number(num);
                    stack.push(exp_num);
                } else {
                    let owned = x.to_string();
                    let err = ParseError::UnexpectedInput(owned);
                    return Err(err);
                }
            }
        }
    };

    assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(res)
}



#[derive(Debug)]
pub enum EvalError {
    DivideByZero,
}

impl Display for EvalError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), FmtError> {
        todo!()
    }
}

fn foo() {
    println!("{}", 5);   // "Display" formatting option
    println!("{:?}", 5); // "Debug" formatting option
}

// "Marker Trait"
impl std::error::Error for EvalError { }

#[derive(Debug)]
pub enum ParseError {
    UnexpectedInput(String),
    InsufficientNumbers,
}

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    match expr {

        Expr::Number(num) => Ok(*num),
        Expr::Sqr(inner_expr) => {
            let number: i64 = eval(inner_expr)?;
            let evald = number * number;
            Ok(evald)
        }
        Expr::Plus(a_expr, b_expr) => {
            let a: i64 = eval(a_expr)?;
            let b: i64 = eval(b_expr)?;

            let evald = a + b;
            Ok(evald)
        }
        Expr::Minus(a_expr, b_expr) => {
            let a: i64 = eval(a_expr)?;
            let b: i64 = eval(b_expr)?;

            let evald = a - b;
            Ok(evald)
        }
        Expr::Multiply(a_expr, b_expr) => {
            let a: i64 = eval(a_expr)?;
            let b: i64 = eval(b_expr)?;

            let evald = a * b;
            Ok(evald)
        }
        Expr::Divide(a_expr, b_expr) => {
            let a: i64 = eval(a_expr)?;
            let b: i64 = eval(b_expr)?;

            if b == 0 {
                return Err(EvalError::DivideByZero);
            }

            let evald = a / b;
            Ok(evald)
        }
    }
}

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
    // use crate::parse; // choice 1
    // use super::parse; // choice 2
    use super::*;     // choice 3

    #[test]
    fn eval_number() {
        let expr = Expr::Number(42);
        let result = eval(&expr).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn eval_sqr() {
        // "3 ^ 2"
        let text = "3 sqr";
        let expected = Expr::Sqr(Box::new(Expr::Number(3)));
        let parsed = parse(text).unwrap();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn eval_add() {
        // "3 + 2"
        let text = "3 2 +";
        let expected = Expr::Plus(
            Box::new(Expr::Number(3)),
            Box::new(Expr::Number(2)),
        );
        let parsed = parse(text).unwrap();
        assert_eq!(parsed, expected);
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
