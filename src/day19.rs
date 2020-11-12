#![allow(dead_code)]
use derive_more::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::*;
use std::fs;
use strsim::levenshtein;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_test() {
        day19();
    }
}

pub fn day19() {
    let input = fs::read_to_string("inputs/day19.txt").unwrap();
    let (xforms, start) = parse(&input);

    let count = part1(&xforms, start);
    assert_eq!(count, 509);

    part2(&xforms, start);
}

fn parse(input: &str) -> (Vec<(String, String)>, &str) {
    let lines = input.lines().collect::<Vec<_>>();
    let start = lines[lines.len() - 1];
    let xforms = &lines[..lines.len() - 2];

    let transformations = xforms
        .into_iter()
        .map(|line| {
            let split = line.split(" => ").collect::<Vec<_>>();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect();

    (transformations, start)
}

fn part1(xforms: &Vec<(String, String)>, start: &str) -> usize {
    let part1: HashSet<String> = xforms
        .into_iter()
        .flat_map(|(source, target)| gen_xforms((source, target), &start))
        .collect();

    part1.len()
}

#[derive(Debug, Eq, PartialEq, Constructor)]
struct Molecule<'a> {
    current: String,
    target: &'a str,
}

impl<'a> PartialOrd for Molecule<'a> {
    fn partial_cmp(&self, other: &Molecule) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Molecule<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let dist_self = levenshtein(&self.current, self.target);
        let dist_other = levenshtein(&other.current, other.target);
        dist_self.cmp(&dist_other).reverse()
    }
}

fn part2(xforms: &Vec<(String, String)>, to_make: &str) {
    let mut queue: BinaryHeap<Molecule> = BinaryHeap::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut count = 0;

    queue.push(Molecule::new("e".to_owned(), to_make));

    while let Some(molecule) = queue.pop() {
        dbg!(&molecule);
        count += 1;
        if count % 10 == 0 {
            println!("Running {}", count);
        }

        if molecule.current == to_make {
            println!("Found it!");
            break;
        }

        let new_molecules: Vec<Molecule> = xforms
            .into_iter()
            .flat_map(|(initial, target)| {
                gen_xforms((initial, target), &molecule.current)
                    .into_iter()
                    .filter(|xform| !seen.contains(xform) && xform.len() <= to_make.len())
                    .map(move |xform| Molecule::new(xform, to_make))
            })
            .collect();

        new_molecules.into_iter().for_each(|molecule| {
            seen.insert(molecule.current.clone());
            queue.push(molecule);
        });
    }
}

fn gen_xforms((source, target): (&String, &String), start: &str) -> Vec<String> {
    let re = Regex::new(&source).unwrap();
    re.find_iter(start)
        .map(|rematch| {
            let s = rematch.start();
            let e = rematch.end();
            format!("{}{}{}", &start[..s], target, &start[e..])
        })
        .collect()
}
