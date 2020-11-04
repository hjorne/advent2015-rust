use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

type E = Box<dyn std::error::Error>;

#[test]
pub fn day13_test() {
    day13().unwrap();
}

#[allow(dead_code)]
pub fn day13() -> Result<(), E> {
    let input = fs::read_to_string("inputs/day13.txt")?;
    let (happymap, people) = parse(&input);

    part1(&happymap, &people);
    part2(&happymap, &people);

    Ok(())
}

fn part1(happymap: &HashMap<(&str, &str), i64>, people: &Vec<&str>) {
    let mut max = 0;

    for permutation in permute::permutations_of(&people) {
        let permutation = permutation.collect::<Vec<_>>();
        let mut total = 0;

        for i in 0..permutation.len() {
            let person1 = permutation[i];
            let person2 = permutation[(i + 1) % permutation.len()];
            total += happymap.get(&(*person1, *person2)).unwrap();
            total += happymap.get(&(*person2, *person1)).unwrap();
        }

        if total > max {
            max = total;
        }
    }

    assert_eq!(max, 618);
    println!("Total happiness is {}", max);
}

fn part2(happymap: &HashMap<(&str, &str), i64>, people: &Vec<&str>) {
    let mut max = 0;

    let mut people = people.clone();
    people.push("me");

    let mut happymap = happymap.clone();
    for p in &people {
        happymap.insert(("me", p), 0);
        happymap.insert((p, "me"), 0);
    }

    for permutation in permute::permutations_of(&people) {
        let permutation = permutation.collect::<Vec<_>>();
        let mut total = 0;

        for i in 0..permutation.len() {
            let person1 = permutation[i];
            let person2 = permutation[(i + 1) % permutation.len()];
            total += happymap.get(&(*person1, *person2)).unwrap();
            total += happymap.get(&(*person2, *person1)).unwrap();
        }

        if total > max {
            max = total;
        }
    }

    println!("Total happiness is {}", max);
}

fn parse(input: &str) -> (HashMap<(&str, &str), i64>, Vec<&str>) {
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
        .unwrap();
    let mut people = HashSet::new();

    let map = re
        .captures_iter(&input)
        .map(|capture| {
            let person1 = capture.get(1).unwrap().as_str();
            let gainloss = capture.get(2).unwrap().as_str();
            let mut amount = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let person2 = capture.get(4).unwrap().as_str();

            if gainloss == "lose" {
                amount = -amount;
            }

            people.insert(person1);
            ((person1, person2), amount)
        })
        .collect();
    (map, people.into_iter().collect())
}
