
// TODO remove after test/debug
// use pest::Parser;
// use pest_derive::Parser;

mod parser;

use parser::{Token, try_it};

const wtf: Token = Token::Not;

fn main() {
    let test_str: &str = "!c + a_1 *! ( c*!_d12) + !( !a_2 *!cdeFG_12_op)";

    try_it(test_str);

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
