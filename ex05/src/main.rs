mod structs;

use crate::structs::{*};

pub fn is_formula_valid(formula: &str) -> Result<Vec<char>, String> {
    let mut count_binary: u32 = 0;
    let mut count_operators: u32 = 0;
    let mut tmp = formula.chars();

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

fn negation_normal_form(formula: &str) -> String {
    match is_formula_valid(formula) {
        Ok(mut variables) => {
            let formula = str::replace(formula, "!!", "");
            let tree_head: Node = Node::create(formula.chars().rev().collect::<String>().as_str(), false);
            variables.sort();
            tree_head.print();
            /*let mut var_values: Vec<bool> = Vec::new();
            var_values.resize(variables.len(), false);
            for val in variables.iter() {
                print!("| {} ", *val)
            }
            println!("| = |");
            for _val in variables.iter() {
                print!("|---")
            }
            println!("|---|");
            truth_table(&tree_head, &variables, &mut var_values, 0);*/
        }
        Err(s) => {
            eprintln!("Error: {}", s);
        }
    }
    return String::from("");
}

fn main() {
    println!("{}", negation_normal_form("AB|C&!"));
}