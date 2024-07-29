
use std::collections::HashSet;

use crate::parser::Expression::{self, *};

type VarMap = Vec<(String, i32)>;
struct CNFRep {
    formula: Vec<HashSet<i32>>,
    trues: Vec<i32>,
}

impl Expression {
    fn expr_to_nnf(self) -> Expression {
        match self {
            Not(expr) => match *expr {
                Not(expr) => expr.expr_to_nnf(),
                Or(expr, expr2) => And(Box::new(Not(expr)), Box::new(Not(expr2))).expr_to_nnf(),
                And(expr, expr2) => Or(Box::new(Not(expr)), Box::new(Not(expr2))).expr_to_nnf(),
                _ => Not(Box::new(expr.expr_to_nnf())),
            },
            And(expr, expr2) => And(Box::new(expr.expr_to_nnf()), Box::new(expr2.expr_to_nnf())),
            Or(expr, expr2) => Or(Box::new(expr.expr_to_nnf()), Box::new(expr2.expr_to_nnf())),
            _ => self,
        }
    }
    fn distribute(expr: Expression, expr2: Expression) -> Expression {
        match expr {
            And(a, b) => And(Box::new(Expression::distribute(*a, expr2.clone())), Box::new(Expression::distribute(*b, expr2))),
            a => match expr2 {
                And(b, c) => And(Box::new(Expression::distribute(a.clone(), *b)), Box::new(Expression::distribute(a, *c))),
                b => Or(Box::new(a), Box::new(b)),
            }
        }
    }
    fn nnf_to_cnf(self) -> Expression {
        match self {
            Not(expr) => Not(Box::new(expr.nnf_to_cnf())),
            And(expr, expr2) => And(Box::new(expr.nnf_to_cnf()), Box::new(expr2.nnf_to_cnf())),
            Or(expr, expr2) => Expression::distribute(expr.nnf_to_cnf(), expr2.nnf_to_cnf()),
            Var(_) => self,
        }
    }

    fn expr_to_cnfrep_helper(self, varmap: &mut VarMap) -> Vec<HashSet<i32>> {
        match self {
            Var(var) => vec!(match lookup(&var, varmap) {
                Some(val) => HashSet::from([val]),
                None => HashSet::from([update(var, varmap)]),
            }),
            Not(expr) => match *expr {
                Var(var) => vec!(match lookup(&var, varmap) {
                    Some(val) => HashSet::from([-val]),
                    None => HashSet::from([-update(var, varmap)]),
                }),
                _ => panic!("Should not occur!"),
            },
            And(expr, expr2) => {
                let mut result = expr.expr_to_cnfrep_helper(varmap);
                result.append(&mut expr2.expr_to_cnfrep_helper(varmap));
                result
            },
            Or(expr, expr2) => {
                let mut result = expr.expr_to_cnfrep_helper(varmap);
                result.append(&mut expr2.expr_to_cnfrep_helper(varmap));
                vec!(result.iter().fold(HashSet::new(), |mut combine, set| { combine.extend(set); combine }))
            },
        }
    }

    fn expr_to_cnf(self) -> Expression {
        self.expr_to_nnf().nnf_to_cnf()
    }
}

fn lookup(value: &String, varmap: &VarMap) -> Option<i32> {
    Some(varmap.iter().find(|(v, _)| value == v)?.1)
}

fn update(var: String, varmap: &mut VarMap) -> i32 {
    let val = match varmap.last() {
        Some((_, val)) => val + 1,
        None => 1,
    };
    varmap.push((var, val));
    val
}

pub fn solve(expr: Expression) -> Vec<Vec<i32>> {
    let mut varmap = Vec::new();
    let cnf = expr.expr_to_cnf();
    println!("{cnf}");
    let formula = cnf.expr_to_cnfrep_helper(&mut varmap);
    println!("{varmap:?}");
    println!("{formula:?}");
    let mut cnfrep = CNFRep { formula, trues: Vec::new() };
    dpll(&mut cnfrep)
}

fn get_status(clause: &HashSet<i32>, trues: &Vec<i32>) -> Status {
    let mut candidates = Vec::new();
    for literal in clause {
        if trues.contains(literal) {
            return Status::True;
        }
        if !trues.contains(&-literal) {
            candidates.push(*literal);
        }
    }
    match candidates.len() {
        0 => Status::False,
        1 => Status::Single(*candidates.last().unwrap()),
        _ => Status::Multiple(candidates),
    }
}

enum Status {
    True,
    False,
    Single(i32),
    Multiple(Vec<i32>),
}

fn dpll(cnfrep: &mut CNFRep) -> Vec<Vec<i32>> {
    'main: loop {

        let mut all_candidates = Vec::new();
    
        for clause in &cnfrep.formula {
            match get_status(clause, &cnfrep.trues) {
                Status::True => continue,
                Status::False => return Vec::new(),
                Status::Single(single) => {
                    cnfrep.trues.push(single);
                    continue 'main;
                }
                Status::Multiple(mut candidates) => all_candidates.append(&mut candidates),
            }
        }

        if all_candidates.is_empty() {
            return vec!(cnfrep.trues.clone());
        }

        let literal = *all_candidates.last().unwrap();

        cnfrep.trues.push(literal);
        let mut result = dpll(cnfrep);
        cnfrep.trues.pop();

        cnfrep.trues.push(-literal);
        result.append(&mut dpll(cnfrep));
        cnfrep.trues.pop();

        return result;
    }
}
