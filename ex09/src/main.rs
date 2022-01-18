mod rules;
mod structs;

use crate::rules::*;
use crate::structs::*;
use std::str::Chars;
use std::collections::HashMap;

pub fn is_formula_valid(formula: &str, set_len: usize) -> Result<Vec<char>, String> {
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
    return if variables.len() != set_len {
        Err(format!("Number of Variables different from Number of Sets"))
    } else if count_binary == count_operators + 1 {
        Ok(variables)
    } else if count_binary == 0 {
        Err(format!("Empty"))
    } else {
        Err(format!("Not Enough operators"))
    };
}

fn create_set_map(variables: Vec<char>, sets: Vec<Vec<i32>>) -> HashMap<char, Vec<i32>> {
    let mut set_map : HashMap<char, Vec<i32>> = HashMap::new();
    for i in 0..sets.len() {
        set_map.insert(variables[i], sets[i].clone());
    }

    return set_map;
}

fn vec_only_dup(arr: Vec<i32>) -> Vec<i32> {
    let mut arr_only_dup: Vec<i32> = Vec::new();
    let mut last_value: i32 = 0;
    for (i, value) in arr.iter().enumerate() {
        if i > 0 && last_value == *value {
            arr_only_dup.push(*value);
        }
        last_value = *value;
    }
    return arr_only_dup;
}

fn eval(value: &Node, set_map: &HashMap<char, Vec<i32>>) -> Vec<i32> {
    match value {
        structs::Node::V(v) => {
            return if v.negation { Vec::new() } else { set_map[&v.variable].clone() };
        },
        structs::Node::E(expression) => {
            let v1_value : Vec<i32> = eval(&expression.value1, set_map);
            let mut v2_value : Vec<i32> = eval(&expression.value2, set_map);

            let res: Vec<i32> = match expression.operator {
                Operator::Conjunction => {
                    let mut tmp: Vec<i32> = v1_value;
                    tmp.append(&mut v2_value);
                    tmp.sort();
                    vec_only_dup(tmp)
                },
                Operator::Disjunction => {
                    let mut tmp: Vec<i32> = v1_value;
                    tmp.append(&mut v2_value);
                    tmp.sort();
                    tmp.dedup();
                    tmp
                },
                _ => Vec::new(),
            };
            return if expression.negation { Vec::new() } else { res };
        },
        structs::Node::N => return Vec::new(),
    }
}

fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    match is_formula_valid(formula, sets.len()) {
        Ok(mut variables) => {
            let formula: String = str::replace(formula, "!!", "");
            let mut tree_head: Node =
                Node::create(formula.chars().rev().collect::<String>().as_str(), false);
            variables.sort();
            let set_map: HashMap<char, Vec<i32>> = create_set_map(variables, sets);
            rule(&mut tree_head);
            return eval(&tree_head, &set_map);
        }
        Err(s) => {
            eprintln!("Error: {}", s);
        }
    }
    return Vec::new();
}

fn print_res(res: Vec<i32>) {
    print!("[");
    for value in res {
        print!(" {}", value);
    }
    println!(" ]");
}

fn main() {
    let sets: Vec<Vec<i32>> = vec!(vec!(0, 1, 2), vec!(0, 3, 4));
    let result = eval_set("AB&", sets);
    print_res(result);
    // [0]

    let sets: Vec<Vec<i32>> = vec!(vec!(0, 1, 2), vec!(3, 4, 5));
    let result = eval_set("AB|", sets);
    print_res(result);
    // [0, 1, 2, 3, 4, 5]

    let sets: Vec<Vec<i32>> = vec!(vec!(0, 1, 2));
    let result = eval_set("A!", sets);
    print_res(result);
    // []
}
