#![allow(dead_code)]

use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn day15() {
        let input = fs::read_to_string("inputs/day15.txt").unwrap();
        let ingredients = parse(&input);
        let part1 = run(&ingredients, false);
        assert_eq!(part1, 21367368);
        let part2 = run(&ingredients, true);
        assert_eq!(part2, 1766400);
    }

    #[test]
    fn day15_dot_test() {
        let result = dot(&vec![1, 2, 3], &vec![4, 5, 6]);
        assert_eq!(result, 32);
    }

    #[test]
    fn day15_transpose_test() {
        let result = transpose(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let desired = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(result, desired);
    }
}

fn run(ingredients: &Vec<Vec<i64>>, count_cals: bool) -> i64 {
    let size = 100;
    let ingredients_t = transpose(ingredients);
    let mut max = 0;

    for i in 0..size + 1 {
        for j in 0..size + 1 - i {
            for k in 0..size + 1 - j {
                let l = size - i - j - k;
                if l > 0 {
                    let weights = vec![i, j, k, l];

                    let score = ingredients_t[0..4]
                        .iter()
                        .map(|vec| dot(vec, &weights))
                        .fold(1, |acc, x| if x > 0 { acc * x } else { 0 });

                    let score = if count_cals {
                        let calories = dot(&ingredients_t[4], &weights);
                        if calories == 500 {
                            score
                        } else {
                            0
                        }
                    } else {
                        score
                    };

                    if score > max {
                        max = score;
                    }
                }
            }
        }
    }

    max
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    let re = Regex::new(
        r"(\w+): \w+ (\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)",
    )
    .unwrap();

    re.captures_iter(input)
        .map(|capture| {
            let capacity = capture.get(2).unwrap().as_str().parse().unwrap();
            let dur = capture.get(3).unwrap().as_str().parse().unwrap();
            let flavor = capture.get(4).unwrap().as_str().parse().unwrap();
            let texture = capture.get(5).unwrap().as_str().parse().unwrap();
            let cals = capture.get(6).unwrap().as_str().parse().unwrap();

            vec![capacity, dur, flavor, texture, cals]
        })
        .collect()
}

fn dot(v1: &Vec<i64>, v2: &Vec<i64>) -> i64 {
    v1.into_iter()
        .zip(v2.into_iter())
        .fold(0, |acc, (&x1, &x2)| acc + x1 * x2)
}

fn mult(c: i64, v: &Vec<i64>) -> Vec<i64> {
    v.into_iter().map(|x| c * x).collect()
}

fn transpose<T: Copy>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = v.len();
    let m = v[0].len();
    let mut new_v = vec![vec![v[0][0]; n]; m];

    for i in 0..v.len() {
        for j in 0..v[i].len() {
            new_v[j][i] = v[i][j];
        }
    }

    new_v
}
