#[derive(PartialEq, Clone)]
pub enum Operator {
    Conjunction,
    Disjunction,
    ExclusiveDisjunction,
    MaterialCondition,
    LogicalEquivalence,
    Wrong,
}

impl Operator {
    pub fn get(c: char) -> Operator {
        return match c {
            '&' => Operator::Conjunction,
            '|' => Operator::Disjunction,
            '^' => Operator::ExclusiveDisjunction,
            '>' => Operator::MaterialCondition,
            '=' => Operator::LogicalEquivalence,
            _ => Operator::Wrong,
        };
    }
}

#[derive(PartialEq, Clone)]
pub struct Expression {
    pub value1: Node,
    pub value2: Node,
    pub operator: Operator,
    pub negation: bool,
}

impl Expression {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!(
            "{}",
            self.to_string()
        );
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.value1.to_string(),
            self.value2.to_string(),
            format!(
                "{}{}",
                match &self.operator {
                    Operator::Conjunction => "&",
                    Operator::Disjunction => "|",
                    Operator::ExclusiveDisjunction => "^",
                    Operator::MaterialCondition => ">",
                    Operator::LogicalEquivalence => "=",
                    Operator::Wrong => "",
                },
                if self.negation { "!" } else { "" }
            )
        )
    }
}

#[derive(PartialEq, Clone)]
pub struct Value {
    pub variable: char,
    pub negation: bool,
}

impl Value {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.variable, if self.negation { "!" } else { "" })
    }
}

use Node::{E, N, V};

#[derive(PartialEq, Clone)]
pub enum Node {
    V(Value),
    E(Box<Expression>),
    N,
}

impl Node {
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        return match self {
            V(value) => value.to_string(),
            E(expression) => expression.to_string(),
            N => format!(""),
        };
    }

    pub fn negate(&mut self) {
        match self {
            V(value) => value.negation ^= true,
            E(expression) => expression.negation ^= true,
            N => {}
        }
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

    pub fn create(formula: &str, negation: bool) -> Node {
        let mut tmp = formula.chars();

        if let Some(c) = tmp.next() {
            match c {
                'A'..='Z' => {
                    return Node::V(Value {
                        variable: c,
                        negation: negation,
                    })
                }
                '&' | '|' | '^' | '>' | '<' | '=' => {
                    let n: usize = Node::expression_length(&formula[1..]) + 1;
                    return Node::E(Box::new(Expression {
                        value1: Node::create(&formula[n..], false),
                        value2: Node::create(&formula[1..n], false),
                        operator: Operator::get(c),
                        negation: negation,
                    }));
                }
                '!' => return Node::create(&formula[1..], true),
                _ => {}
            }
        }
        return Node::N;
    }
}
