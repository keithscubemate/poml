use pest_derive::Parser;

use pest::{
    Parser,
    iterators::Pair,
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
});

fn main() -> io::Result<()> {
    print!("> ");
    io::stdout().flush()?;
    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.is_empty() {
            print!("> ");
            io::stdout().flush()?;
            continue;
        }

        match Calculator::parse(Rule::file, &line) {
            Ok(pairs) => {
                for pair in pairs {
                    if matches!(pair.as_rule(), Rule::EOI) {
                        continue;
                    }

                    let parsed = parse_expr(pair);
                    println!("Parsed {:#?}", parsed);
                    // println!("Evaled {:#?}", eval(&parsed));
                }
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
    BinOp {
        lhs: Box<Expr>,
        op: Oper,
        rhs: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        texp: Box<Expr>,
        fexp: Box<Expr>,
    },
    App {
        fexp: Box<Expr>,
        aexp: Box<Expr>,
    },
    Lam {
        arg: String,
        body: Box<Expr>,
    },
    Let {
        bind: String,
        body: Box<Expr>,
    },
    LetRec {
        bind: String,
        body: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Oper {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::integer => Expr::Integer(pair.as_str().parse().unwrap()),
        Rule::ident => Expr::Var(pair.as_str().to_string()),
        Rule::if_expr => {
            let mut pairs = pair.into_inner();
            if pairs.len() != 3 {
                panic!("unreachable rn")
            }

            let cond = pairs.next().map(parse_expr).map(|e| Box::new(e)).unwrap();
            let texp = pairs.next().map(parse_expr).map(|e| Box::new(e)).unwrap();
            let fexp = pairs.next().map(parse_expr).map(|e| Box::new(e)).unwrap();

            Expr::If { cond, texp, fexp }
        }
        Rule::app_expr => {
            let mut pairs = pair.into_inner().map(parse_expr);

            let first = pairs.next().unwrap();

            pairs.fold(first, |acc, arg| Expr::App {
                fexp: Box::new(acc),
                aexp: Box::new(arg),
            })
        }
        Rule::lam_expr => {
            let mut pairs = pair.into_inner().rev();

            let body = pairs.next().map(parse_expr).map(|e| Box::new(e)).unwrap();

            let mut args = pairs.map(|r| r.as_str());

            let frarg = args.next().unwrap();

            let mut lam = Expr::Lam {
                arg: frarg.to_string(),
                body,
            };

            for rarg in args {
                lam = Expr::Lam {
                    arg: rarg.to_string(),
                    body: Box::new(lam),
                }
            }

            lam
        }
        Rule::let_expr => {
            let mut pairs = pair.into_inner();
            if pairs.len() != 2 {
                panic!("unreachable rn")
            }

            let bind = pairs.next().unwrap().as_str().to_string();
            let body = pairs.next().map(parse_expr).map(|e| Box::new(e)).unwrap();

            Expr::Let { bind, body }
        }
        Rule::let_rec_expr => {
            let mut pairs = pair.into_inner();
            if pairs.len() != 2 {
                panic!("unreachable rn")
            }

            let bind = pairs.next().unwrap().as_str().to_string();
            let body = pairs.next().map(parse_expr).map(|e| Box::new(e)).unwrap();

            Expr::LetRec { bind, body }
        }
        Rule::equation => PRATT_PARSER
            .map_primary(|prim| match prim.as_rule() {
                Rule::integer => Expr::Integer(prim.as_str().parse().unwrap()),
                Rule::ident => Expr::Var(prim.as_str().to_string()),
                Rule::expr | Rule::app_expr | Rule::if_expr => parse_expr(prim),
                rule => unreachable!(
                    "Expr::parse expected atom, found {:?}: {:?}",
                    rule,
                    prim.as_str()
                ),
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
            .parse(pair.into_inner()),
        Rule::EOI => Expr::Var("that's all".to_string()),
        rule => unreachable!("Unexpected rule {:?}", rule),
    }
}

fn eval(ast: &Expr) -> i32 {
    match ast {
        Expr::Integer(i) => *i,
        Expr::BinOp { lhs, op, rhs } => match op {
            Oper::Add => eval(lhs) + eval(rhs),
            Oper::Subtract => eval(lhs) - eval(rhs),
            Oper::Multiply => eval(lhs) * eval(rhs),
            Oper::Divide => eval(lhs) / eval(rhs),
        },
        Expr::Var(_) => todo!(),
        Expr::If { .. } => todo!(),
        Expr::App { .. } => todo!(),
        Expr::Lam { .. } => todo!(),
        Expr::Let { .. } => todo!(),
        Expr::LetRec { .. } => todo!(),
    }
}
