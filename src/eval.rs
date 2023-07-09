use crate::parser;
use std::collections::HashSet;

use parser::Expression;

// Find all unique variables in expression.
pub fn find_variables(expr: &Expression) -> HashSet<&str> {
    let mut vars = HashSet::new();
    traverse_expression_tree(expr, &mut vars);
    vars
}

fn traverse_expression_tree<'a>(expr: &'a Expression, vars: &mut HashSet<&'a str>) {
    match expr {
        Expression::Var(v) => {
            vars.insert(v);
        }
        Expression::Not(e) => traverse_expression_tree(e.as_ref(), vars),
        Expression::Or(lhs, rhs) => {
            traverse_expression_tree(lhs.as_ref(), vars);
            traverse_expression_tree(rhs.as_ref(), vars);
        }
        Expression::And(lhs, rhs) => {
            traverse_expression_tree(lhs.as_ref(), vars);
            traverse_expression_tree(rhs.as_ref(), vars);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn get_unique_vars() {
        let expr = parse("a | aa & a_1 | (! aaa_123)").unwrap();
        let vars = find_variables(&expr);
        assert_eq!(vars, HashSet::from(["a", "aa", "a_1", "aaa_123"]))
    }

    #[test]
    fn get_repetitive_vars() {
        let expr = parse("b | a & a | (! b) & b | (a | b)").unwrap();
        let vars = find_variables(&expr);
        assert_eq!(vars, HashSet::from(["a", "b"]))
    }
}
