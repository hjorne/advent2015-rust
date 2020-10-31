use regex::Regex;
use std::cmp::max;
use std::fs;

enum InstrType {
    Toggle,
    TurnOff,
    TurnOn,
}

#[derive(Copy, Clone, Debug)]
struct Point(usize, usize);

impl Point {
    fn between(self, other: Point) -> impl Iterator<Item = Point> {
        (self.0..other.0 + 1).flat_map(move |x| (self.1..other.1 + 1).map(move |y| Point(x, y)))
    }
}

struct Instr {
    instr_type: InstrType,
    start: Point,
    end: Point,
}

pub fn day6() {
    let input = fs::read_to_string("inputs/day6.txt").unwrap();
    let input = parser(&input);

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Instr>) {
    let size = 1000;
    let mut grid = vec![vec![0; size]; size];

    input.into_iter().for_each(|instr| match instr.instr_type {
        InstrType::Toggle => instr.start.between(instr.end).for_each(|point| {
            grid[point.0][point.1] = 1 - grid[point.0][point.1];
        }),
        InstrType::TurnOff => instr.start.between(instr.end).for_each(|point| {
            grid[point.0][point.1] = 0;
        }),
        InstrType::TurnOn => instr.start.between(instr.end).for_each(|point| {
            grid[point.0][point.1] = 1;
        }),
    });

    let count = grid
        .into_iter()
        .map(|subgrid| subgrid.into_iter().sum::<u32>())
        .sum::<u32>();
    println!("{} lights are on", count);
}

fn part2(input: &Vec<Instr>) {
    let size = 1000;
    let mut grid = vec![vec![0; size]; size];

    input.into_iter().for_each(|instr| match instr.instr_type {
        InstrType::Toggle => instr.start.between(instr.end).for_each(|point| {
            grid[point.0][point.1] += 2;
        }),
        InstrType::TurnOff => instr.start.between(instr.end).for_each(|point| {
            grid[point.0][point.1] = max(0, grid[point.0][point.1] - 1);
        }),
        InstrType::TurnOn => instr.start.between(instr.end).for_each(|point| {
            grid[point.0][point.1] += 1;
        }),
    });

    let count = grid
        .into_iter()
        .map(|subgrid| subgrid.into_iter().sum::<i32>())
        .sum::<i32>();
    println!("The total brightness is {}", count);
}

fn parser(input: &str) -> Vec<Instr> {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let instr_type = match captures.get(1).unwrap().as_str() {
                "toggle" => InstrType::Toggle,
                "turn on" => InstrType::TurnOn,
                "turn off" => InstrType::TurnOff,
                _ => panic!("Unknown type"),
            };
            let x1 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let y1 = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let x2 = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let y2 = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

            Instr {
                instr_type,
                start: Point(x1, y1),
                end: Point(x2, y2),
            }
        })
        .collect()
}
