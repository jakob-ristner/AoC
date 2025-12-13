use rayon::prelude::*;
use xcov::{DlxBuilder, ExactCoverProblem, MrvExactCoverSearch};

mod model;
use model::*;

mod parser;
use parser::parse;

mod display;
use display::*;

/*
* Can be made very much faster by just determining
* the satisfiability on if grid area >= total shape area
* and skipping the DLX solving entirely.
* But this is pretty to look at.
*/

fn main() {
    let input = include_str!("../input.txt");
    let (_, instances) = parse(input).unwrap();
    let solutions: Vec<_> = instances
        .par_iter()
        .filter_map(|instance| {
            let (instance, solution) = solve(instance)?;
            print_solution_colored(&solution, instance.grid_width, instance.grid_height);
            println!();
            Some((instance, solution))
        })
        .collect();
    let ans = solutions.len();
    println!("Total solutions found: {}", ans);
}

fn solve(instance: &Instance) -> Option<(Instance, Vec<Vec<usize>>)> {
    let total_area: usize = instance
        .shapes
        .iter()
        .map(|(shape, count)| shape.area() * count)
        .sum();

    if total_area > instance.grid_width * instance.grid_height {
        return None;
    }

    let mut builder = DlxBuilder::new(
        instance.grid_width * instance.grid_height
            + instance
                .shapes
                .iter()
                .map(|(_, count)| count)
                .sum::<usize>(),
        0,
    );

    let mut piece_index = 0;
    for (shape, _count) in instance.shapes.iter() {
        let options = shape.all_variant_options(instance.grid_width, instance.grid_height);
        for _ in 0..*_count {
            let options = Shape::set_piece_constraint(
                options.clone(),
                instance.grid_width,
                instance.grid_height,
                piece_index,
            );
            piece_index += 1;
            for option in &options {
                builder.add_option(option);
            }
        }
    }

    for i in 1..=instance.grid_width * instance.grid_height {
        builder.add_option(&[i]);
    }

    let mut problem = MrvExactCoverSearch::new(builder.build());
    problem.search();

    if let Some(solution) = problem.current_solution() {
        let solution = solution
            .into_iter()
            .map(|opt| opt.to_vec())
            .collect::<Vec<_>>();
        return Some((instance.clone(), solution));
    }
    None
}
