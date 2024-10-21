use compute::operations::circuits::builder_orig::CircuitBuilder;
use compute::uint::GarbledUint16;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use tandem::GateIndex;

#[derive(Parser)]
#[grammar = "boolean_expr.pest"] // points to the pest grammar file
struct BooleanExprParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(xor_op, Left))
            .op(Op::infix(and_op, Left))
            .op(Op::prefix(not_op))
    };
}

pub struct CircuitEvaluator<'a, const N: usize> {
    builder: &'a mut CircuitBuilder<N>,
}

#[derive(Debug)]
pub enum Expr {
    Variable(String),
    UnaryNot(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Xor,
    And,
}

impl<'a, const N: usize> CircuitEvaluator<'a, N> {
    pub fn new(builder: &'a mut CircuitBuilder<N>) -> Self {
        Self { builder }
    }

    pub fn build_circuit(
        &mut self,
        expr: Expr,
        vars: &mut HashMap<String, GateIndex>,
    ) -> GateIndex {
        match expr {
            Expr::Variable(name) => *vars.get(&name).expect("Unknown variable"),
            Expr::UnaryNot(expr) => {
                let index = self.build_circuit(*expr, vars);
                self.builder.push_not(index)
            }
            Expr::BinOp { lhs, op, rhs } => {
                let left_index = self.build_circuit(*lhs, vars);
                let right_index = self.build_circuit(*rhs, vars);
                match op {
                    Op::Xor => self.builder.push_xor(left_index, right_index),
                    Op::And => self.builder.push_and(left_index, right_index),
                }
            }
        }
    }
}

pub fn parse_expr(pairs: pest::iterators::Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::ident => Expr::Variable(primary.as_str().to_string()),
            Rule::boolean_expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::xor_op => Op::Xor,
                Rule::and_op => Op::And,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::not_op => Expr::UnaryNot(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

fn main() {
    let lhs: GarbledUint16 = 547_u16.into();
    let rhs: GarbledUint16 = 8390_u16.into();

    let mut builder = CircuitBuilder::<16>::new(vec![&lhs, &rhs]);

    // Define variable inputs
    let mut vars = HashMap::new();
    vars.insert("a".to_string(), 0); // Assuming gate index 0 is "a"
    vars.insert("b".to_string(), 1); // Assuming gate index 1 is "b"

    let input_expr = "(a ^ b) ^ (a & b)";
    match BooleanExprParser::parse(Rule::equation, input_expr) {
        Ok(mut pairs) => {
            let expr = parse_expr(pairs.next().unwrap().into_inner());
            let mut evaluator = CircuitEvaluator::new(&mut builder);
            let result_gate = evaluator.build_circuit(expr, &mut vars);
            println!("Final OR gate index: {}", result_gate);
        }
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
        }
    }
}
