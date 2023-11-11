use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "./calculator/calculator.pest"]
pub struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;
        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left) | Op::infix(power, Left))
            .op(Op::prefix(unary_minus))
    };
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

pub fn calculate(expr: String) -> Result<f64, String> {
    match CalculatorParser::parse(Rule::equation, &expr) {
        Ok(mut pairs) => Ok(eval(parse_expr(pairs.next().unwrap().into_inner()))),
        Err(e) => Err(e.to_string()),
    }
}

fn eval(expr: Expr) -> f64 {
    match expr {
        Expr::Number(n) => n,
        Expr::UnaryMinus(e) => -eval(*e),
        Expr::BinOp { lhs, op, rhs } => {
            let (e1, e2) = (eval(*lhs), eval(*rhs));
            match op {
                Op::Add => e1 + e2,
                Op::Subtract => e1 - e2,
                Op::Multiply => e1 * e2,
                Op::Divide => e1 / e2,
                Op::Modulo => e1 % e2,
                Op::Power => e1.powf(e2),
            }
        }
    }
}

fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Expr::Number(primary.as_str().parse::<f64>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                Rule::modulo => Op::Modulo,
                Rule::power => Op::Power,
                _ => unreachable!(),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}
