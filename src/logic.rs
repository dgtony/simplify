use std::collections::HashMap;
use std::iter::{repeat, zip};

use quine_mc_cluskey::Bool as QBool;

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

// Simplify given logical expression and return all found minimal
// solutions converted into its string representation.
//
// Current version uses external crate quine-mc_cluskey for simplification.
// Type conversions add some non-negligible overhead especially in case of
// dense truth tables, which could be eliminated by using another more
// efficient implementation, e.g. based on:
//
//   Udovenko, Aleksei. "DenseQMC: an efficient bit-slice implementation of
//   the Quine-McCluskey algorithm." arXiv preprint arXiv:2302.10083 (2023).
pub fn simplify(expr: &Expression) -> Option<Vec<String>> {
    let (labels, ones) = truth_table_ones(expr)?;
    let qmc_exp = encode(ones, labels.len() as u8);

    Some(
        qmc_exp
            .simplify()
            .iter()
            .map(|solution| decode(solution, &labels))
            .collect(),
    )
}

// Turn vector of minterms into qmc expression.
fn encode(ones: Vec<usize>, num_terms: u8) -> QBool {
    let mut terms: Vec<QBool> = Vec::new();
    for one in ones {
        let mut mt = Vec::new();
        for i in 0..num_terms {
            if (one & 1 << i) > 0 {
                mt.push(QBool::Term(i))
            } else {
                mt.push(QBool::Not(Box::new(QBool::Term(i))))
            }
        }
        terms.push(QBool::And(mt));
    }
    QBool::Or(terms)
}

// Turn resulting qmc expression into string.
fn decode(e: &QBool, labels: &[String]) -> String {
    match e {
        QBool::False => "false".to_string(),
        QBool::True => "true".to_string(),
        QBool::Term(i) => labels[*i as usize].clone(),
        QBool::Not(inner) => format!("!{}", decode(inner.as_ref(), labels)),
        QBool::And(inner) => {
            let decoded: Vec<String> = inner.iter().map(|s| decode(s, labels)).collect();
            decoded.join(" & ")
        }
        QBool::Or(inner) => {
            let decoded: Vec<String> = inner.iter().map(|s| decode(s, labels)).collect();
            decoded.join(" | ")
        }
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
