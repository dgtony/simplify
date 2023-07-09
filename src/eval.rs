use crate::parser;
use parser::Expression;
use std::collections::{HashMap, HashSet};

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
        Expression::Not(e) => traverse_expression_tree(e, vars),
        Expression::Or(lhs, rhs) => {
            traverse_expression_tree(lhs, vars);
            traverse_expression_tree(rhs, vars);
        }
        Expression::And(lhs, rhs) => {
            traverse_expression_tree(lhs, vars);
            traverse_expression_tree(rhs, vars);
        }
    };
}

// Evaluate expression with given set of values for variables.
// Variables missing in set are considered as a false ones.
pub fn evaluate(expr: &Expression, vars: &HashMap<&str, bool>) -> bool {
    match expr {
        Expression::Var(v) => *vars.get(v.as_str()).or(Some(&false)).unwrap(),
        Expression::Not(e) => !evaluate(e, vars),
        Expression::Or(lhs, rhs) => evaluate(lhs, vars) | evaluate(rhs, vars),
        Expression::And(lhs, rhs) => evaluate(lhs, vars) & evaluate(rhs, vars),
    }
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

    #[test]
    fn eval() {
        struct TestCase<'a> {
            vars: HashMap<&'a str, bool>,
            expected: bool,
        }

        let expr = parse("(a | b) & !c | [!d & a | c & !b]").unwrap();

        let test_cases: Vec<TestCase> = Vec::from([
            TestCase {
                vars: HashMap::from([("a", true), ("b", true), ("c", true), ("d", true)]),
                expected: false,
            },
            TestCase {
                vars: HashMap::from([("a", true), ("b", false), ("c", true), ("d", false)]),
                expected: true,
            },
            TestCase {
                vars: HashMap::from([("a", false), ("b", true), ("c", false), ("d", false)]),
                expected: true,
            },
            TestCase {
                vars: HashMap::new(),
                expected: false,
            },
        ]);

        for (i, tt) in test_cases.iter().enumerate() {
            let result = evaluate(&expr, &tt.vars);
            assert_eq!(
                result,
                tt.expected,
                "case {} -> expected: {:?}, got: {:?}",
                i + 1,
                tt.expected,
                result
            )
        }
    }
}
