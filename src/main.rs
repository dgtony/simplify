// TODO remove after test/debug
// use pest::Parser;
// use pest_derive::Parser;

mod eval;
mod parser;

use parser::parse;

use std::process::exit;

fn main() {
    let test_str: &str =
        "!!c \n | a_1  *! ( \n c  &! _d12) | [(12a & !__12)  ] + !( !a_2 *!cdeFG_12_op)";
    // let test_str: &str = "\n";

    match parse(test_str) {
        Ok(ast) => {
            println!("[OK] expression AST:\n{:#?}", ast);
        }
        Err(e) => {
            println!("[ERR] parsing expression: {}", e);
            exit(1);
        }
    }

    // let test_str = " a_1 *! ( c*!_d12) + !( !a_2 *!cdeFG_12_op)";

    // let pairs = LogicParser::parse(Rule::expr, test_str).expect("WTF?");
    // for pair in pairs.into_iter() {
    //     println!("{:#?}", pair)
    // }
}

// TODO impl:
//  1. Expression tokenizer
//  2. Expression parser
//  3. AST builder (optional compilation ?)
//  4. Quine-McCluskey logic simplifier
