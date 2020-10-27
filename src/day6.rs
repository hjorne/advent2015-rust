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
    input
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<_>>();

            let (instr_type, start, end) = if line.starts_with("toggle") {
                (
                    InstrType::Toggle,
                    str_to_point(split[1]),
                    str_to_point(split.last().unwrap()),
                )
            } else if line.starts_with("turn off") {
                (
                    InstrType::TurnOff,
                    str_to_point(split[2]),
                    str_to_point(split.last().unwrap()),
                )
            } else if line.starts_with("turn on") {
                (
                    InstrType::TurnOn,
                    str_to_point(split[2]),
                    str_to_point(split.last().unwrap()),
                )
            } else {
                panic!("Unknown string {}", line);
            };
            Instr {
                instr_type,
                start,
                end,
            }
        })
        .collect()
}

fn str_to_point(s: &str) -> Point {
    let split: Vec<usize> = s.split(",").map(|string| string.parse().unwrap()).collect();
    Point(split[0], split[1])
}
