use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;

struct Equation {
    test_value: i64,
    terms: VecDeque<i64>,
}

fn input() -> Vec<Equation> {
    let mut equations = vec![];

    let filename = env::args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        let mut iter = line.split(':');
        let part1 = iter.next().unwrap();
        let part2 = iter.next().unwrap();

        let test_value = part1.parse::<i64>().unwrap();
        let terms: VecDeque<i64> = part2
            .trim()
            .split_whitespace()
            .map(|x| -> i64 { x.parse().unwrap() })
            .collect();

        equations.push(Equation { test_value, terms });
    }

    equations
}

fn evaluate_equation(equation: &Equation) -> bool {
    if equation.terms.len() == 1 {
        return equation.terms[0] == equation.test_value;
    }

    let mut plus_terms: VecDeque<i64> = equation.terms.clone();
    let first_term = plus_terms.pop_front().unwrap();
    let second_term = plus_terms.pop_front().unwrap();
    plus_terms.push_front(first_term + second_term);

    let mut mul_terms: VecDeque<i64> = equation.terms.clone();
    let first_term = mul_terms.pop_front().unwrap();
    let second_term = mul_terms.pop_front().unwrap();
    mul_terms.push_front(first_term * second_term);

    evaluate_equation(&Equation {
        test_value: equation.test_value,
        terms: plus_terms,
    }) || evaluate_equation(&Equation {
        test_value: equation.test_value,
        terms: mul_terms,
    })
}

fn main() {
    let equations = input();

    let mut sum = 0;
    for eq in equations.iter() {
        let result = evaluate_equation(eq);
        if result {
            sum += eq.test_value;
            println!("passed: {}", eq.test_value);
        } else {
            println!("failed: {}", eq.test_value);
        }
    }
    println!("total: {}", sum);
}
