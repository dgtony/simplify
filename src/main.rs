mod eval;
mod parser;
mod simp;

use parser::parse;

use std::process::exit;

fn main() {
    let test_str: &str =
        "!!c \n | a_1  *! ( \n c  &! _d12) | [(12a & !__12)  ] + !( !a_2 *!cdeFG_12_op)";

    match parse(test_str) {
        Ok(ast) => {
            println!("[OK] expression AST:\n{:#?}", ast);
        }
        Err(e) => {
            println!("[ERR] parsing expression: {}", e);
            exit(1);
        }
    }
}

// TODO impl:
//  - Quine-McCluskey logic simplifier
