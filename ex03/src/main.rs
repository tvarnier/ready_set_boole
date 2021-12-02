fn is_formula_valid(formula: &str) -> Result<(), String> {
    let mut count_binary: u32 = 0;
    let mut count_operators: u32 = 0;
    let mut tmp = formula.chars();

    let mut i: u32 = 0;
    while let Some(c) = tmp.next() {
        match c {
            '0' | '1' => count_binary += 1,
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
        Ok(())
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
    V(bool),
    E(Box<Expression>),
}

fn expression_length(formula: &str) -> usize {
    let mut tmp = formula.chars();

    let mut i: usize = 0;
    let mut l: u32 = 1;
    while let Some(c) = tmp.next() {
        match c {
            '0' | '1' => l -= 1,
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
            '0' => return Value::V(false),
            '1' => return Value::V(true),
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
                    v2: Value::V(true),
                    operator: c,
                }))
            }
            _ => {}
        }
    }
    return Value::V(true);
}

fn eval(value: Value) -> bool {
    match value {
        Value::V(v) => return v,
        Value::E(e) => {
            let tmp_v1: bool = match e.v1 {
                Value::V(value1) => value1,
                Value::E(_) => eval(e.v1),
            };
            let tmp_v2: bool = match e.v2 {
                Value::V(value2) => value2,
                Value::E(_) => eval(e.v2),
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

fn eval_formula(formula: &str) -> bool {
    match is_formula_valid(formula) {
        Ok(()) => {
            let v: Value = create_tree(formula.chars().rev().collect::<String>().as_str());
            return eval(v);
        }
        Err(s) => {
            eprintln!("Error: {}", s);
            return false;
        }
    }
}

fn main() {
    println!("{}\n", eval_formula("10&"));
    // false
    println!("{}\n", eval_formula("10|"));
    // true
    println!("{}\n", eval_formula("11>"));
    // true
    println!("{}\n", eval_formula("10="));
    // false
    println!("{}\n", eval_formula("1!011|&="));
    // true
}
