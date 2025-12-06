use nom::character::complete::{space1, u32};
use nom::multi::separated_list1;
use nom::{character::complete::line_ending, IResult};

fn main() {
    let contents = include_str!("input.txt");
    let (_, data) = parse(contents).unwrap();

    let p1 = data.iter().filter(|x| is_safe(x)).count();
    let p2 = data.iter().filter(|x| can_remove1(x)).count();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, separated_list1(space1, u32))(input)
}

fn can_remove1(report: &[u32]) -> bool {
    if is_safe(report) {
        return true;
    } 
    for i in 0..report.len() {
        let mut new_report = report.to_owned();
        new_report.remove(i);
        if is_safe(&new_report) {
            return true;
        }
    }
    false
}

fn is_safe(report: &[u32]) -> bool {
    let inc = report[0] < report[1];
    for x in report.windows(2) {
        if (inc && (x[0] >= x[1] || x[1] - x[0] > 3)) || (!inc && (x[1] >= x[0] || x[0] - x[1] > 3)) {
            return false;
        } 
    }
    true
}
