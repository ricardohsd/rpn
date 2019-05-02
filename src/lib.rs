use std::fmt;

const OPERATORS: [&str; 4] = ["+", "-", "*", "/"];

#[derive(Debug, PartialEq)]
pub enum CalcError {
    InvalidOperator,
    InvalidRightSide,
    InvalidLeftSide,
    EvaluationError,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let side = match *self {
            CalcError::InvalidOperator => "operator",
            CalcError::InvalidRightSide => "right side",
            CalcError::InvalidLeftSide => "left side",
            CalcError::EvaluationError => "evaluation error",
        };
        write!(f, "Failed to parse {} value", side)
    }
}

pub struct Calculator {
    stack: Vec<f64>,
}

impl Calculator {
    pub fn run(expression: &str) -> Result<f64, CalcError> {
        let mut calc = Calculator { stack: Vec::new() };

        for e in expression.split_whitespace() {
            let token = e.trim();

            if OPERATORS.contains(&token) {
                let b: f64 = match calc.stack.pop() {
                    Some(i) => i,
                    _ => return Err(CalcError::InvalidRightSide),
                };
                let a: f64 = match calc.stack.pop() {
                    Some(i) => i,
                    _ => return Err(CalcError::InvalidLeftSide),
                };

                let result: f64 = Calculator::execute(token, a, b);

                calc.stack.push(result);
            } else {
                match token.parse::<f64>() {
                    Ok(n) => calc.stack.push(n),
                    Err(_) => return Err(CalcError::InvalidOperator),
                }
            }
        }

        if calc.stack.len() != 1 {
            return Err(CalcError::EvaluationError);
        }

        match calc.stack.pop() {
            Some(i) if i.is_infinite() || i.is_nan() => Err(CalcError::EvaluationError),
            Some(i) => return Ok(i),
            _ => return Err(CalcError::EvaluationError),
        }
    }

    fn execute(token: &str, a: f64, b: f64) -> f64 {
        match token {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => panic!("{} not a valid operator", token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_expression(expression: &str, expected_result: f64) {
        let result = Calculator::run(expression);

        match result {
            Ok(n) => assert_eq!(n, expected_result),
            Err(e) => assert!(false, e),
        }
    }

    #[test]
    fn valid_expressions() {
        assert_expression("3 4 +", 7.0);
        assert_expression("-3.0 4 +", 1.0);
        assert_expression("3.5 1.3 +", 4.8);
        assert_expression("3 4 + 2 *", 14.0);
        assert_expression("3 5 * 3 /", 5.0);
        assert_expression("15 7 1 1 + - / 3 * 2 1 1 + + -", 5.0);
    }

    #[test]
    fn invalid_expressions() {
        let result = Calculator::run("3 + 4");
        match result {
            Ok(_) => assert!(false, "expression should have failed"),
            Err(e) => assert_eq!(e, CalcError::InvalidLeftSide),
        }

        let result = Calculator::run("3 4 + 2");
        match result {
            Ok(_) => assert!(false, "expression should have failed"),
            Err(e) => assert_eq!(e, CalcError::EvaluationError),
        }

        let result = Calculator::run("3, 4 +");
        match result {
            Ok(_) => assert!(false, "expression should have failed"),
            Err(e) => assert_eq!(e, CalcError::InvalidOperator),
        }

        let result = Calculator::run("5 0 /");
        match result {
            Ok(_) => assert!(false, "division by zero, expression should have failed"),
            Err(e) => assert_eq!(e, CalcError::EvaluationError),
        }
    }
}