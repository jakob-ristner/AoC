use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u32};
use nom::multi::separated_list1;
use nom::IResult;

type Regs = [usize; 3];

fn main() {
    let content = include_str!("input.txt");
    let (_, (instr, goal, regs)) = parse(content).unwrap();
    let p1 = printout(&run(&instr, &regs, None));
    let p2 = part_2(&instr, &regs, &goal, goal.len() - 1, 0).unwrap();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn pad_end(v: Vec<usize>, to: usize) -> Vec<usize> {
    let len = v.len();
    let mut res = v;
    let ext = vec![0; to - len];
    res.extend(ext);
    res
}

fn part_2(
    instr: &[Instr],
    registers: &Regs,
    goal: &[usize],
    exp: usize,
    number: usize,
) -> Option<usize> {
    let len = goal.len();
    let mut adj: Vec<usize> = vec![];
    for i in 0..8 {
        let regs = *registers;
        let next = number + i * 8_usize.pow(exp as u32);
        let res = run(instr, &regs, Some(next));
        let res = pad_end(res, len);
        if res[exp] == goal[exp] {
            adj.push(next);
        }
    }
    if exp == 0 {
        return adj.iter().min().copied();
    }
    let mut sols = vec![];
    for a in adj {
        if let Some(res) = part_2(instr, registers, goal, exp - 1, a) {
            sols.push(res);
        }
    }
    sols.iter().min().copied()
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc,
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

fn cb(combo: usize, registers: &Regs) -> usize {
    match combo {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => unreachable!(),
    }
}

fn printout(output: &[usize]) -> String {
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn run(instr: &[Instr], registers: &Regs, a_override: Option<usize>) -> Vec<usize> {
    let mut pc = 0;
    let registers: &mut Regs = &mut registers.clone();
    registers[0] = a_override.unwrap_or(registers[0]);
    let mut output: Vec<usize> = Vec::new();
    while pc < instr.len() {
        match instr[pc] {
            Instr::Adv(c) => registers[0] /= 2_usize.pow(cb(c, registers) as u32),
            Instr::Bxl(l) => registers[1] ^= l,
            Instr::Bst(c) => registers[1] = cb(c, registers) % 8,
            Instr::Jnz(l) => {
                pc = if registers[0] != 0 { l } else { pc + 1 };
                continue;
            }
            Instr::Bxc => registers[1] ^= registers[2],
            Instr::Out(c) => output.push(cb(c, registers) % 8),
            Instr::Bdv(c) => registers[1] = registers[0] / 2_usize.pow(cb(c, registers) as u32),
            Instr::Cdv(c) => registers[2] = registers[0] / 2_usize.pow(cb(c, registers) as u32),
        }
        pc += 1;
    }
    output
}

fn parse(input: &str) -> IResult<&str, (Vec<Instr>, Vec<usize>, Regs)> {
    let (input, _) = tag("Register A: ")(input)?;
    let (input, a) = u32(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Register B: ")(input)?;
    let (input, b) = u32(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Register C: ")(input)?;
    let (input, c) = u32(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Program: ")(input)?;
    let (_, instr) = separated_list1(tag(","), instr)(input)?;
    let (input, numbers) = separated_list1(tag(","), u32)(input)?;
    let numbers = numbers.iter().map(|x| *x as usize).collect();
    Ok((
        input,
        (instr, numbers, [a as usize, b as usize, c as usize]),
    ))
}

fn instr(input: &str) -> IResult<&str, Instr> {
    let (input, inscode) = u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, operand) = u32(input)?;
    let instr = match inscode {
        0 => Instr::Adv(operand as usize),
        1 => Instr::Bxl(operand as usize),
        2 => Instr::Bst(operand as usize),
        3 => Instr::Jnz(operand as usize),
        4 => Instr::Bxc,
        5 => Instr::Out(operand as usize),
        6 => Instr::Bdv(operand as usize),
        7 => Instr::Cdv(operand as usize),
        _ => unreachable!(),
    };
    Ok((input, instr))
}
