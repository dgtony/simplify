mod eval;
mod parser;
mod logic;

use std::env;
use std::process::exit;

use parser::parse;
use logic::simplify;

fn main() {
    let expr = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("provide boolean expression to simplify");
            exit(0);
        }
    };

    let ast = match parse(&expr) {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("[ERR] expression parsing: {}", e);
            exit(1);
        }
    };

    let solutions = match simplify(&ast) {
        Some(ss) => ss,
        None => {
            println!("[ERR] cannot simplify: too many variables");
            exit(1);
        }
    };

    println!("[OK] {} solution(s) found", solutions.len());
    for solution in solutions {
        println!("=> {}", solution);
    }
}
