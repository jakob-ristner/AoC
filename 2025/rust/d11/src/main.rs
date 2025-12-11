use fxhash::FxHashMap;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace0, multispace1, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
};

fn main() {
    let input = include_str!("../input.txt");
    let (_, hm) = parse(input).unwrap();

    let part1 = solve(&hm, true);
    let part2 = solve(&hm, false);
    println!("Part 1: {}", part1);
    println!("Part 1: {}", part2);
}

type Graph<'a> = FxHashMap<&'a str, Vec<&'a str>>;

fn line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, from) = alpha1(input)?;
    let (input, _) = (tag(":"), multispace1).parse(input)?;
    let (input, to) = separated_list1(space1, alpha1).parse(input)?;
    Ok((input, (from, to)))
}

fn parse<'a>(input: &'a str) -> IResult<&'a str, Graph<'a>> {
    let (input, lines) =
        all_consuming(terminated(separated_list1(line_ending, line), multispace0)).parse(input)?;
    let hm = lines
        .into_iter()
        .fold(FxHashMap::default(), |mut hm, (from, to)| {
            hm.insert(from, to);
            hm
        });
    Ok((input, hm))
}

fn solve(graph: &Graph, p1: bool) -> u64 {
    type Memo<'a> = FxHashMap<(&'a str, bool, bool), u64>;
    fn dp<'a>(
        from: &'a str,
        to: &str,
        graph: &'a Graph,
        memo: &mut Memo<'a>,
        dac: bool,
        fft: bool,
    ) -> u64 {
        if from == to && dac && fft {
            return 1;
        }

        let dac = dac || from == "dac";
        let fft = fft || from == "fft";

        if let Some(n) = memo.get(&(from, dac, fft)) {
            return *n;
        }

        let mut acc = 0;
        if let Some(adj) = graph.get(from) {
            for next in adj {
                acc += dp(next, to, graph, memo, dac, fft);
            }
        }
        memo.insert((from, dac, fft), acc);
        acc
    }
    dp(
        if p1 { "you" } else { "svr" },
        "out",
        graph,
        &mut Memo::default(),
        p1,
        p1,
    )
}
