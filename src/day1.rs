use std::fs;

pub fn day1() {
    let s = fs::read_to_string("inputs/day1.txt").expect("Unable to read file");
    part1(&s);
    part2(&s);
}

fn paren_to_value(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn part1(input: &String) {
    let count = input.chars().fold(0, |acc, c| acc + paren_to_value(c));

    println!("Count = {}", count);
}

fn part2(input: &String) {
    let mut count = 0;
    for (i, c) in input.chars().enumerate() {
        count += paren_to_value(c);
        if count < 0 {
            println!("Went negative at {}", i + 1);
            break;
        }
    }
}
