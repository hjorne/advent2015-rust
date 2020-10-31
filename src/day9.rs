use permute::permutations_of;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn day9() {
    let input = fs::read_to_string("inputs/day9.txt").unwrap();
    let edges = parse(&input);
    let verts = edges_to_verts(&edges);

    let mut min = edges.values().sum();
    let mut max = 0;

    permutations_of(verts.as_slice()).for_each(|permutation| {
        let permutation: Vec<&&str> = permutation.collect();
        let mut total = 0;

        for i in 0..permutation.len() - 1 {
            total += edges.get(&(permutation[i], permutation[i + 1])).unwrap();
        }

        if total < min {
            min = total;
        }

        if total > max {
            max = total;
        }
    });

    println!("The minimum dist is {}", min);
    println!("The maximum dist is {}", max);
}

fn parse<'a>(input: &'a String) -> HashMap<(&'a str, &'a str), usize> {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut map = HashMap::new();

    re.captures_iter(&input).for_each(|capture| {
        let start = capture.get(1).unwrap().as_str();
        let end = capture.get(2).unwrap().as_str();
        let dist = capture.get(3).unwrap().as_str().parse().unwrap();

        map.insert((start, end), dist);
        map.insert((end, start), dist);
    });

    map
}

fn edges_to_verts<'a>(edges: &HashMap<(&'a str, &'a str), usize>) -> Vec<&'a str> {
    let mut set = HashSet::new();

    edges.keys().for_each(|key| {
        set.insert(key.0.clone());
        set.insert(key.1.clone());
    });

    set.into_iter().collect()
}
