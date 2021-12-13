use crate::structs::*;

// ¬(A ∧ B) ⇔ (¬A ∨ ¬B)
fn morgan_law_con_to_dis(expression: &mut Expression) -> bool {
    if expression.negation == true && expression.operator == Operator::Conjunction {
        expression.negation ^= true;
        expression.operator = Operator::Disjunction;
        expression.value1.negate();
        expression.value2.negate();
        return true;
    }
    return false;
}

// ¬(A ∨ B) ⇔ (¬A ∧ ¬B)
fn morgan_law_dis_to_con(expression: &mut Expression) -> bool {
    if expression.negation == true && expression.operator == Operator::Disjunction {
        expression.negation ^= true;
        expression.operator = Operator::Conjunction;
        expression.value1.negate();
        expression.value2.negate();
        return true;
    }
    return false;
}

// (A ⇔ B) ⇔ ((A ⇒ B) ∧ (B ⇒ A))
fn equivalence_to_con(expression: &mut Expression) -> bool {
    if expression.operator == Operator::LogicalEquivalence {
        expression.operator = Operator::Conjunction;
        let tmp1: Node = expression.value1.clone();
        let tmp2: Node = expression.value2.clone();
        expression.value1 = Node::E(Box::new(Expression {
            value1: tmp1.clone(),
            value2: tmp2.clone(),
            operator: Operator::MaterialCondition,
            negation: false,
        }));
        expression.value2 = Node::E(Box::new(Expression {
            value1: tmp1,
            value2: tmp2,
            operator: Operator::MaterialCondition,
            negation: false,
        }));
        return true;
    }
    return false;
}

// (A ⇒ B) ⇔ (¬A ∨ B)
fn cond_to_dis(expression: &mut Expression) -> bool {
    if expression.operator == Operator::MaterialCondition {
        expression.operator = Operator::Disjunction;
        expression.value1.negate();
        return true;
    }
    return false;
}

// A ⊕ B <=> (A ∧ ¬B) ∨ (¬A ∧ B)
// A ⊕ B <=> (A ∨ B) ∧ (¬A ∨ ¬B)

// A ⊕ B <=> (A ∨ B) ∧ ¬(A ∧ B)
fn excl_to_con(expression: &mut Expression) -> bool {
    if expression.operator == Operator::ExclusiveDisjunction {
        expression.operator = Operator::Conjunction;
        let tmp1: Node = expression.value1.clone();
        let tmp2: Node = expression.value2.clone();
        expression.value1 = Node::E(Box::new(Expression {
            value1: tmp1.clone(),
            value2: tmp2.clone(),
            operator: Operator::Disjunction,
            negation: false,
        }));
        expression.value2 = Node::E(Box::new(Expression {
            value1: tmp1,
            value2: tmp2,
            operator: Operator::Conjunction,
            negation: true,
        }));
        return true;
    }
    return false;
}

pub fn rule(n: &mut Node) {
    match n {
        Node::E(expression) => {
            equivalence_to_con(expression);
            cond_to_dis(expression);
            excl_to_con(expression);
            morgan_law_con_to_dis(expression);
            morgan_law_dis_to_con(expression);
            rule(&mut expression.value1);
            rule(&mut expression.value2);
        }
        _ => {}
    }
}

// A ∧ (A ∨ B) = A
// A ∨ (A ∧ B) = A
fn absorption(n: &mut Node) -> bool {
    if let Node::E(expression) = n {
        if expression.operator == Operator::Conjunction {
            if let Node::E(ev2) = &expression.value2 {
                if ev2.operator == Operator::Disjunction && (expression.value1 == ev2.value1 || expression.value1 == ev2.value2) {
                    *n = expression.value1.clone();
                    return true;
                }
            }
            if let Node::E(ev1) = &expression.value1 {
                if ev1.operator == Operator::Disjunction && (expression.value2 == ev1.value1 || expression.value2 == ev1.value2) {
                    *n = expression.value2.clone();
                    return true;
                }
            }
        }
        else if expression.operator == Operator::Disjunction {
            if let Node::E(ev2) = &expression.value2 {
                if ev2.operator == Operator::Conjunction && (expression.value1 == ev2.value1 || expression.value1 == ev2.value2) {
                    *n = expression.value1.clone();
                    return true;
                }
            }
            if let Node::E(ev1) = &expression.value1 {
                if ev1.operator == Operator::Conjunction && (expression.value2 == ev1.value1 || expression.value2 == ev1.value2) {
                    *n = expression.value2.clone();
                    return true;
                }
            }
        }
    }
    return false;
}

// A ∨ A = A
// A ∧ A = A
fn idempotence(n: &mut Node) -> bool {
    if let Node::E(expression) = n {
        if matches!(expression.operator, Operator::Conjunction | Operator::Disjunction) && expression.value1 == expression.value2 {
            *n = expression.value1.clone();
        }
    }
    return false;
} 

pub fn rule2(n: &mut Node) {
    if let Node::E(expr) = n {
        rule2(&mut expr.value1);
        rule2(&mut expr.value2);
    }
    idempotence(n);
    absorption(n);
}