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

fn conjunctive_normal_form(formula: &str) -> String {
    match is_formula_valid(formula) {
        Ok(mut variables) => {
            let formula: String = str::replace(formula, "!!", "");
            let mut tree_head: Node =
                Node::create(formula.chars().rev().collect::<String>().as_str(), false);
            variables.sort();
            rule(&mut tree_head);
            rule2(&mut tree_head);
            return tree_head.to_string();
        }
        Err(s) => {
            eprintln!("Error: {}", s);
        }
    }
    return String::from("");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", conjunctive_normal_form(&args[1]));
}
