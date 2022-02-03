fn is_formula_valid(formula: &str) -> Result<Vec<char>, String> {
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

struct Expression {
    v1: Value,
    v2: Value,
    operator: char,
}

enum Value {
    V(char),
    E(Box<Expression>),
}

fn expression_length(formula: &str) -> usize {
    let mut tmp = formula.chars();

    let mut i: usize = 0;
    let mut l: u32 = 1;
    while let Some(c) = tmp.next() {
        match c {
            'A'..='Z' => l -= 1,
            '&' | '|' | '^' | '>' | '=' => l += 1,
            _ => (),
        };
        i += 1;
        if l == 0 {
            return i;
        }
    }
    return i;
}

fn create_tree(formula: &str) -> Value {
    let mut tmp = formula.chars();

    if let Some(c) = tmp.next() {
        match c {
            'A'..='Z' => return Value::V(c),
            '&' | '|' | '^' | '>' | '<' | '=' => {
                let n: usize = expression_length(&formula[1..]) + 1;
                return Value::E(Box::new(Expression {
                    v1: create_tree(&formula[n..]),
                    v2: create_tree(&formula[1..n]),
                    operator: c,
                }));
            }
            '!' => {
                return Value::E(Box::new(Expression {
                    v1: create_tree(&formula[1..]),
                    v2: Value::V(' '),
                    operator: c,
                }))
            }
            _ => {}
        }
    }
    return Value::V(' ');
}

fn eval(value: &Value, variables: &Vec<char>, var_values: &Vec<bool>) -> bool {
    match value {
        Value::V(v) => return var_values[variables.iter().position(|&r| r == *v).unwrap()],
        Value::E(e) => {
            let tmp_v1: bool = match e.v1 {
                Value::V(value1) => {
                    var_values[variables.iter().position(|&r| r == value1).unwrap()]
                }
                Value::E(_) => eval(&e.v1, variables, var_values),
            };
            let tmp_v2: bool = match e.v2 {
                Value::V(value2) => {
                    if value2 != ' ' {
                        var_values[variables.iter().position(|&r| r == value2).unwrap()]
                    } else {
                        false
                    }
                }
                Value::E(_) => eval(&e.v2, variables, var_values),
            };
            let res: bool = match e.operator {
                '!' => !tmp_v1,
                '&' => tmp_v1 & tmp_v2,
                '|' => tmp_v1 | tmp_v2,
                '^' => tmp_v1 ^ tmp_v2,
                '>' => {
                    if tmp_v1 == true && tmp_v2 == false {
                        false
                    } else {
                        true
                    }
                }
                '=' => tmp_v1 == tmp_v2,
                _ => false,
            };
            return res;
        }
    }
}

fn truth_table(tree_head: &Value, variables: &Vec<char>, var_values: &mut Vec<bool>, l: usize) {
    var_values[l] = false;
    if l + 1 == variables.len() {
        for val in var_values.iter() {
            print!("| {} ", *val as u8)
        }
        println!("| {} |", eval(tree_head, variables, &var_values) as u8);
    } else {
        truth_table(tree_head, variables, var_values, l + 1)
    }

    var_values[l] = true;
    if l + 1 == variables.len() {
        for val in var_values.iter() {
            print!("| {} ", *val as u8)
        }
        println!("| {} |", eval(tree_head, variables, &var_values) as u8);
    } else {
        truth_table(tree_head, variables, var_values, l + 1)
    }
}

fn print_truth_table(formula: &str) {
    match is_formula_valid(formula) {
        Ok(mut variables) => {
            let tree_head: Value = create_tree(formula.chars().rev().collect::<String>().as_str());
            variables.sort();
            let mut var_values: Vec<bool> = Vec::new();
            var_values.resize(variables.len(), false);
            for val in variables.iter() {
                print!("| {} ", *val)
            }
            println!("| = |");
            for _val in variables.iter() {
                print!("|---")
            }
            println!("|---|");
            truth_table(&tree_head, &variables, &mut var_values, 0);
        }
        Err(s) => {
            eprintln!("Error: {}", s);
        }
    }
}

fn main() {
    println!("\nAB&");
    print_truth_table("AB&");
    println!("\nAB|");
    print_truth_table("AB|");
    println!("\nAB^");
    print_truth_table("AB^");
    println!("\nAB>");
    print_truth_table("AB>");
    println!("\nAB=");
    print_truth_table("AB=");
    println!("\nABC!&|DEF^>=");
    print_truth_table("ABC!&|DEF^>=");
}
