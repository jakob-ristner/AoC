mod parser;
use good_lp::*;
use itertools::Itertools;
use parser::*;
use rayon::prelude::*;

fn main() {
    let input = include_str!("../input.txt");
    let (_, machines) = parse(input).unwrap();

    let p1 = machines.par_iter().map(p1).sum::<usize>();
    let p2 = machines.par_iter().map(p2).sum::<usize>();

    println!("Part 1: {}", p1);
    println!("Part 1: {}", p2);
}

fn p1(machine: &Machine) -> usize {
    let ind_len = machine.indicators.len();
    let buttons_len = machine.buttons.len();
    for num_combs in 1..=buttons_len {
        for combo in machine.buttons.iter().combinations(num_combs) {
            if (0..ind_len)
                .all(|d| combo.iter().fold(0, |acc, b| acc ^ b[d]) == machine.indicators[d])
            {
                return num_combs;
            }
        }
    }
    unreachable!()
}

fn p2(machine: &Machine) -> usize {
    let mut vars = ProblemVariables::new();

    let mut presses = Vec::new();
    for _ in 0..machine.buttons.len() {
        let variable = vars.add(variable().min(0).integer());
        presses.push(variable);
    }

    let mut problem = microlp(vars.minimise(presses.iter().sum::<Expression>()));

    for d in 0..machine.joltage.len() {
        let mut expr = Expression::from(0.0);
        for (i, button) in machine.buttons.iter().enumerate() {
            expr.add_mul(button[d] as f64, presses[i]);
        }
        problem.add_constraint(expr.eq(machine.joltage[d] as f64));
    }
    let sol = problem.solve().unwrap();
    presses
        .into_iter()
        .map(|v| sol.value(v).round() as usize)
        .sum()
}
