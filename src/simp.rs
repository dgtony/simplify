use std::collections::HashMap;
use std::fmt::Error;
use std::iter::{repeat, zip};

use crate::eval::{evaluate, find_variables};
use crate::parser::Expression;

// Evaluate given expression on all possible inputs and return
// * list of "ones" - combinations of arguments evaluating expression
//   to true, encoded in the form of iteration number, and
// * stable var order - list of variable names used for both truth
//   table encoding and expression simplification.
pub fn truth_table_ones(expr: &Expression) -> Option<(Vec<String>, Vec<usize>)> {
    let vars_set = find_variables(expr);
    if vars_set.len() > 32 {
        // give up fast, problem is too hard
        return None;
    }

    // hashmap with expression argument's values reused during evaluation
    let mut vars: HashMap<&str, bool> = HashMap::from_iter(zip(vars_set, repeat(false)));

    // stable order of variables
    let mut vars_stable: Vec<String> =
        Vec::from_iter(vars.keys().into_iter().map(|k| k.to_string()));
    vars_stable.sort();

    let mut ones: Vec<usize> = Vec::new();

    // brute force entire state space
    let iterations: usize = usize::pow(2, vars_stable.len() as u32);
    for i in 0..iterations {
        // TODO add progress bar

        fill_vars(vars_stable.as_slice(), &mut vars, i);
        if evaluate(expr, &vars) {
            ones.push(i);
        }
    }
    Some((vars_stable, ones))
}

#[inline]
fn fill_vars<'a>(
    stable_order: &'a [String],
    vars: &mut HashMap<&'a str, bool>,
    num_variant: usize,
) {
    let mut num_v = num_variant;
    for bit in 0..stable_order.len() {
        let value: bool = num_v % 2 == 1;
        let var: &str = &stable_order[bit];
        vars.insert(var, value);
        num_v = num_v / 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn var_filling() {
        let order: Vec<String> = vec!["a", "b", "c", "d"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let mut vars: HashMap<&str, bool> = HashMap::new();

        fill_vars(&order, &mut vars, 0);
        assert_eq!(
            vars,
            HashMap::from_iter([("a", false), ("b", false), ("c", false), ("d", false)])
        );

        fill_vars(&order, &mut vars, 1);
        assert_eq!(
            vars,
            HashMap::from_iter([("a", true), ("b", false), ("c", false), ("d", false)])
        );

        fill_vars(&order, &mut vars, 2);
        assert_eq!(
            vars,
            HashMap::from_iter([("a", false), ("b", true), ("c", false), ("d", false)])
        );

        fill_vars(&order, &mut vars, 7);
        assert_eq!(
            vars,
            HashMap::from_iter([("a", true), ("b", true), ("c", true), ("d", false)])
        );
    }

    #[test]
    fn ones_simple() {
        let expr = parse("a & (b | !c)").unwrap();
        let (vars, ones) = truth_table_ones(&expr).unwrap();

        assert_eq!(vars, vec!["a", "b", "c"]);
        assert_eq!(ones, vec![1, 3, 7]);
    }

    #[test]
    fn ones_complex() {
        let expr = parse("(!a & (b & !d)) | (a & (e & d) & !c)").unwrap();
        let (vars, ones) = truth_table_ones(&expr).unwrap();

        assert_eq!(vars, vec!["a", "b", "c", "d", "e"]);
        assert_eq!(ones, vec![2, 6, 18, 22, 25, 27]);
    }
}
