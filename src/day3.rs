use std::collections::HashSet;
use std::fs;

pub fn day3() {
    let input = fs::read_to_string("inputs/day3.txt").expect("Couldn't read file");
    let input = input.trim();

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    input.chars().fold((0, 0), |pos, c| {
        let new_pos = move_pos(pos, c);
        seen.insert(new_pos);
        new_pos
    });
    println!("{} houses got visited", seen.len());
}

fn part2(input: &str) {
    let mut seen = HashSet::new();
    let mut santa = (0, 0);
    let mut robot_santa = (0, 0);
    seen.insert((0, 0));

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            santa = move_pos(santa, c);
            seen.insert(santa);
        } else {
            robot_santa = move_pos(robot_santa, c);
            seen.insert(robot_santa);
        }
    }

    println!("{} houses got visited", seen.len());
}

fn move_pos(pos: (i32, i32), dir: char) -> (i32, i32) {
    let (x, y) = pos;
    match dir {
        '<' => (x - 1, y),
        '>' => (x + 1, y),
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        _ => panic!("Unknown character match"),
    }
}
