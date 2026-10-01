use pest_derive::Parser;

use pest::{
    Parser,
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};
use std::{
    io::{self, BufRead, Write},
    sync::LazyLock,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Calculator;

static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
        .op(Op::prefix(Rule::unary_minus))
});

fn main() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()?;
    for line in io::stdin().lock().lines() {
        match Calculator::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                let parsed = parse_expr(pairs.next().unwrap().into_inner());
                println!("Parsed {:#?}", parsed);

                // println!("Evaled {:#?}", eval(&parsed));
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
        print!("> ");
        io::stdout().flush()?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    Var(String),
    Prefix {
        op: Oper,
        rhs: Box<Expr>,
    },
    BinOp {
        lhs: Box<Expr>,
        op: Oper,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Oper {
    Add,
    Subtract,
    UnaryMinus,
    Multiply,
    Divide,
}

fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|prim| match prim.as_rule() {
            Rule::integer => Expr::Integer(prim.as_str().parse().unwrap()),
            Rule::ident => Expr::Var(prim.as_str().to_string()),
            Rule::expr => parse_expr(prim.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_prefix(|op, rhs| {
            let op = match op.as_rule() {
                Rule::unary_minus => Oper::UnaryMinus,
                rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
            };

            Expr::Prefix {
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Oper::Add,
                Rule::sub => Oper::Subtract,
                Rule::mul => Oper::Multiply,
                Rule::div => Oper::Divide,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };

            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .parse(pairs)
}

fn eval(ast: &Expr) -> i32 {
    match ast {
        Expr::Integer(i) => *i,
        Expr::Prefix { op, rhs } => match op {
            Oper::UnaryMinus => -1 * eval(rhs),
            _ => panic!(),
        },
        Expr::BinOp { lhs, op, rhs } => match op {
            Oper::Add => eval(lhs) + eval(rhs),
            Oper::Subtract => eval(lhs) - eval(rhs),
            Oper::Multiply => eval(lhs) * eval(rhs),
            Oper::Divide => eval(lhs) / eval(rhs),
            _ => panic!(),
        },
        Expr::Var(_) => todo!(),
    }
}
