use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::{
        complete::{alphanumeric1, char, line_ending, not_line_ending, space1, usize},
        streaming::multispace0,
    },
    multi::{many_m_n, separated_list1},
    sequence::separated_pair,
};

use super::model::{Instance, Shape};

fn shape(input: &str) -> IResult<&str, Shape> {
    let (input, _) = (alphanumeric1, tag(":"), line_ending).parse(input)?;
    let (input, shape_str) =
        many_m_n(3, 3, not_line_ending.and(line_ending).map(|(line, _)| line)).parse(input)?;
    let shape = Shape::from(shape_str.join("\n").as_str());
    let (input, _) = multispace0(input)?;
    Ok((input, shape))
}

fn shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    separated_list1(multispace0, shape).parse(input)
}

fn instance(shapes: &[Shape]) -> impl Fn(&str) -> IResult<&str, Instance> + '_ {
    move |input: &str| {
        let (input, (grid_width, grid_height)) =
            separated_pair(usize, char('x'), usize).parse(input)?;
        let (input, _) = (char(':'), space1).parse(input)?;
        let (input, counts) = separated_list1(space1, usize).parse(input)?;
        let shapes_with_counts = shapes
            .iter()
            .copied()
            .zip(counts)
            .filter(|&(_, count)| count > 0)
            .collect();
        Ok((
            input,
            Instance {
                shapes: shapes_with_counts,
                grid_width,
                grid_height,
            },
        ))
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Instance>> {
    let (input, shapes) = shapes(input)?;
    let (input, _) = multispace0(input)?;
    let (input, instances) = separated_list1(line_ending, instance(&shapes)).parse(input)?;
    Ok((input, instances))
}
