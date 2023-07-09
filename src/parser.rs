extern crate pest;

use pest::error::Error;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;

// Common Pratt-parser for correct processing of operator
// precedence and associativity not expressed in a grammar.
lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc, Op};

        // define operation precedence from lowest to highest
        PrattParser::new()
            .op(Op::infix(Rule::or, Assoc::Left))
            .op(Op::infix(Rule::and, Assoc::Left))
            .op(Op::prefix(Rule::not))
    };
}

// Boolean expression parser autogenerated from grammar file.
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LogicParser;

// Parse string with logical expression into AST.
pub fn parse(expression: &str) -> Result<AstNode, Error<Rule>> {
    // Raw parsing result, used as a source for Pratt parser.
    let mut pairs = LogicParser::parse(Rule::logexpr, expression)?;

    // starting from the inner of expr
    Ok(parse_expr(pairs.next().unwrap().into_inner()))
}

// Transform plain grammar-level tree into proper AST.
fn parse_expr(pairs: Pairs<Rule>) -> AstNode {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::var => AstNode::Var(primary.as_str().to_string()),
            Rule::expr => parse_expr(primary.into_inner()), // from "(" ~ expr ~ ")"
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::not => AstNode::Not(Box::new(rhs)),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::and => AstNode::And(Box::new(lhs), Box::new(rhs)),
            Rule::or => AstNode::Or(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Var(String),
    Not(Box<AstNode>),
    Or(Box<AstNode>, Box<AstNode>),
    And(Box<AstNode>, Box<AstNode>),
}

// Part 1: lexing
pub fn tokenize(_input: &str) -> Result<Vec<Token>, &str> {
    todo!("tokenize input expression");
}

// Part 2: parsing
// TODO

// Part 3: constructing AST
// TODO
