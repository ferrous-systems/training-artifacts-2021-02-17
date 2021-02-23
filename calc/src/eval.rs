use std::fmt::{
    Formatter,
    Error as FmtError,
    Display
};

use crate::Expr;

#[derive(Debug)]
pub enum EvalError {
    DivideByZero,
}


impl Display for EvalError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), FmtError> {
        todo!()
    }
}

// "Marker Trait"
impl std::error::Error for EvalError { }


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


#[cfg(test)]
mod tests {
    // use crate::parse; // choice 1
    // use super::parse; // choice 2
    use super::*;     // choice 3
    use crate::parse::parse;

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
}
