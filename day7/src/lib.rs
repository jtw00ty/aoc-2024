use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum Operator {
    Plus,
    Times,
    Concat,
}

impl Operator {
    fn operate(&self, a: f64, b: f64) -> f64 {
        match self {
            Self::Plus => a + b,
            Self::Times => a * b,
            Self::Concat => a* 10_f64.powf(b.log10().floor() + 1_f64) + b,
        }
    }
}

#[derive(Debug)]
pub struct Equation {
    pub value: f64,
    pub operands: Vec<f64>,
}

impl Equation {
    pub fn find_operators(&self) -> Option<Vec<Operator>> {
        let mut out = vec![];
        if self._find_operators(&mut out) {
            Some(out)
        } else {
            None
        }
    }

    pub fn find_operators_concat(&self) -> Option<Vec<Operator>> {
        let mut out = vec![];
        if self._find_operators_concat(&mut out) {
            Some(out)
        } else {
            None
        }
    }

    fn _find_operators_concat(&self, operators: &mut Vec<Operator>) -> bool {
        if operators.len() == self.operands.len() - 1 {
            self.eval(operators) == self.value
        } else {
            operators.push(Operator::Plus);
            if self._find_operators_concat(operators) {
                return true;
            }
            operators.pop();

            operators.push(Operator::Times);
            if self._find_operators_concat(operators) {
                return true;
            }
            operators.pop();

            operators.push(Operator::Concat);
            if self._find_operators_concat(operators) {
                return true;
            }
            operators.pop();

            false
        }
    }

    fn _find_operators(&self, operators: &mut Vec<Operator>) -> bool {
        if operators.len() == self.operands.len() - 1 {
            self.eval(operators) == self.value
        } else {
            operators.push(Operator::Plus);
            if self._find_operators(operators) {
                return true;
            }
            operators.pop();

            operators.push(Operator::Times);
            if self._find_operators(operators) {
                return true;
            }
            operators.pop();

            false
        }
    }

    fn eval(&self, operators: &Vec<Operator>) -> f64 {
        let mut operands = self.operands.iter();
        let operators = operators.iter();
        let first = operands.next().unwrap();

        operands
            .zip(operators)
            .fold(*first, |acc, (val, op)| op.operate(acc, *val))
    }
}

pub fn read_input<P>(path: P) -> Vec<Equation>
where
    P: AsRef<std::path::Path>,
{
    let lines = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap());

    lines
        .map(|line| {
            let (val, ops) = line.split_once(": ").unwrap();
            let value = val.parse().unwrap();
            let operands = ops
                .split(' ')
                .map(|operand| operand.parse().unwrap())
                .collect();
            Equation { value, operands }
        })
        .collect()
}
