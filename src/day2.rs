use std::fs;

struct Box(u32, u32, u32);

pub fn day2() {
    let input = fs::read_to_string("inputs/day2.txt").expect("Could not read file");
    let input = parse(input);

    part1(&input);
    part2(&input);
}

fn parse(input: String) -> Vec<Box> {
    input
        .trim()
        .split("\n")
        .map(|s| {
            let split: Vec<u32> = s
                .split("x")
                .map(|s_inner| s_inner.parse()
                     .expect("Could not parse integer"))
                .collect();

            Box(split[0], split[1], split[2])
        })
        .collect()
}

fn part1(input: &Vec<Box>) {
    let sum: u32 = input
        .iter()
        .map(paper_needed)
        .sum();

    println!("The total amount of paper needed is {}", sum);
}

fn part2(input: &Vec<Box>) {
    let sum: u32 = input
        .iter()
        .map(ribbon_needed)
        .sum();

    println!("The total amount of paper needed is {}", sum);
}

fn paper_needed(b: &Box) -> u32 {
    let Box(l, w, h) = b;
    let area: u32 = two_smallest_sides(b).into_iter().product();

    2 * l * w + 2 * w * h + 2 * h * l + area
}

fn ribbon_needed(b: &Box) -> u32 {
    let Box(l, w, h) = b;
    let v = two_smallest_sides(b);
    2 * v[0] + 2 * v[1] + l * w * h
}

fn two_smallest_sides(b: &Box) -> Vec<u32> {
    let Box(l, w, h) = b;
    let mut v = vec![*l, *w, *h];
    v.sort();
    v.pop();
    v
}
