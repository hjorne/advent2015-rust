#![allow(dead_code)]
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn day16() {
        let target: HashMap<&str, i64> = vec![
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]
        .into_iter()
        .collect();

        let input = fs::read_to_string("inputs/day16.txt").unwrap();
        let sues = parse(&input);

        let part1 = part1(&sues, &target);
        assert_eq!(part1, 213);

        let part2 = part2(&sues, &target);
        assert_eq!(part2, 323);
    }
}

fn part1(sues: &Vec<HashMap<String, i64>>, target: &HashMap<&str, i64>) -> i64 {
    let sues_remaining: Vec<_> = sues
        .into_iter()
        .filter(|sue| {
            for (key, val) in target {
                let target_val = sue.get(*key).unwrap_or(val);

                if val != target_val {
                    return false;
                }
            }
            true
        })
        .collect();

    sues_remaining[0]["Sue number"]
}

fn part2(sues: &Vec<HashMap<String, i64>>, target: &HashMap<&str, i64>) -> i64 {
    let sues_remaining: Vec<_> = sues
        .into_iter()
        .filter(|sue| {
            for (key, detect_val) in target {
                let key = *key;
                let real_val = sue.get(key);
                if real_val.is_none() {
                    continue;
                }
                let real_val = real_val.unwrap();

                if key == "cats" || key == "trees" {
                    if detect_val >= real_val {
                        return false;
                    }
                } else if key == "pomeranians" || key == "goldfish" {
                    if detect_val <= real_val {
                        return false;
                    }
                } else {
                    if detect_val != real_val {
                        return false;
                    }
                }
            }
            true
        })
        .collect();
    sues_remaining[0]["Sue number"]
}

fn parse(input: &str) -> Vec<HashMap<String, i64>> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let line = line.replace(":", "").replace(",", "");
            let split: Vec<&str> = line.split_whitespace().collect();
            vec![
                ("Sue number".to_owned(), split[1].parse().unwrap()),
                (split[2].to_owned(), split[3].parse().unwrap()),
                (split[4].to_owned(), split[5].parse().unwrap()),
                (split[6].to_owned(), split[7].parse().unwrap()),
            ]
            .into_iter()
            .collect()
        })
        .collect()
}
