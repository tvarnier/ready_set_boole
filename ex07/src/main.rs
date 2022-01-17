mod rules;
mod structs;

use crate::rules::*;
use crate::structs::*;
use std::env;
use std::str::Chars;

pub fn is_formula_valid(formula: &str) -> Result<Vec<char>, String> {
    let mut count_binary: u32 = 0;
    let mut count_operators: u32 = 0;
    let mut tmp: Chars = formula.chars();

    let mut variables: Vec<char> = Vec::new();

    let mut i: u32 = 0;
    while let Some(c) = tmp.next() {
        match c {
            'A'..='Z' => {
                count_binary += 1;
                if !variables.contains(&c) {
                    variables.push(c);
                }
            }
            '&' | '|' | '^' | '>' | '=' => {
                count_operators += 1;
                if count_binary == count_operators {
                    return Err(format!("Pos {} | Operator Exceed or Equal Numbers", i));
                }
            }
            '!' => {
                if i == 0 {
                    return Err(format!("Pos {} | Cannot begin with `!`", i));
                }
            }
            _ => return Err(format!("Pos {} | Character {} not valid", i, c)),
        }
        i += 1;
    }
    return if count_binary == count_operators + 1 {
        Ok(variables)
    } else if count_binary == 0 {
        Err(format!("Empty"))
    } else {
        Err(format!("Not Enough operators"))
    };
}

fn eval(value: &Node, variables: &Vec<char>, var_values: &Vec<bool>) -> bool {
    match value {
        structs::Node::V(v) => {
            let res: bool = var_values[variables.iter().position(|&r| r == v.variable).unwrap()];
            return if v.negation { !res } else { res };
        },
        structs::Node::E(expression) => {
            let v1_value : bool = eval(&expression.value1, variables, var_values);
            let v2_value : bool = eval(&expression.value2, variables, var_values);

            let res: bool = match expression.operator {
                Operator::Conjunction => v1_value & v2_value,
                Operator::Disjunction => v1_value | v2_value,
                _ => false,
            };
            return if expression.negation { !res } else { res };
        },
        structs::Node::N => return false,
    }
}

fn loop_values(tree_head: &Node, variables: &Vec<char>, var_values: &mut Vec<bool>, l: usize) -> bool {
    var_values[l] = false;
    if l + 1 == variables.len() {
        if eval(tree_head, variables, &var_values) { return true; }
    } else {
        if loop_values(tree_head, variables, var_values, l + 1) { return true; }
    }

    var_values[l] = true;
    if l + 1 == variables.len() {
        if eval(tree_head, variables, &var_values) { return true; }
    } else {
        if loop_values(tree_head, variables, var_values, l + 1) { return true; }
    }

    return false;
}

fn is_satisfiable(tree_head: Node, variables: Vec<char>) -> bool {
    let mut var_values: Vec<bool> = Vec::new();
    var_values.resize(variables.len(), false);
    return loop_values (&tree_head, &variables, &mut var_values, 0);
}

fn sat(formula: &str) -> bool {
    match is_formula_valid(formula) {
        Ok(mut variables) => {
            let formula: String = str::replace(formula, "!!", "");
            let mut tree_head: Node =
                Node::create(formula.chars().rev().collect::<String>().as_str(), false);
            variables.sort();
            rule(&mut tree_head);
            tree_head.print();
            return is_satisfiable(tree_head, variables);
        }
        Err(s) => {
            eprintln!("Error: {}", s);
        }
    }
    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", sat(&args[1]));
}
