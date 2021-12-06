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

pub struct Expression {
    pub value1: Node,
    pub value2: Node,
    pub operator: Operator,
    pub negation: bool,
}

impl Expression {
    pub fn print(&self) {
        self.value1.print();
        self.value2.print();
        print!("{}{}",
            match &self.operator {
                Operator::Conjunction => "&",
                Operator::Disjunction => "|",
                Operator::ExclusiveDisjunction => "^",
                Operator::MaterialCondition => ">",
                Operator::LogicalEquivalence => "=",
                Operator::Wrong => "",
            },
            if self.negation { "!" } else { "" }
        );
    }
}

pub struct Value {
    pub variable: char,
    pub negation: bool,
}

impl Value {
    pub fn print(&self) {
        print!("{}{}", self.variable, if self.negation { "!" } else { "" });
    }
}

use Node::{V, E, N};

pub enum Node {
    V(Value),
    E(Box<Expression>),
    N,
}

impl Node {
    pub fn print(&self) {
        match self {
            V(value) => value.print(),
            E(expression) => expression.print(),
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
                'A'..='Z' => return Node::V(Value {
                    variable: c,
                    negation: negation,
                }),
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
                _ => {},
            }
        }
        return Node::N;
    }
}