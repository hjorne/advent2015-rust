#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn day18() {
        let input = fs::read_to_string("inputs/day18.txt").unwrap();
        let part1 = part1(&input);
        let part2 = part2(&input);
        assert_eq!(part1, 768);
        assert_eq!(part2, 781);
    }

    #[test]
    fn day18_neighbors_test() {
        assert_eq!(neighbors((0, 0), (5, 5)).collect::<Vec<_>>().len(), 3);
        assert_eq!(neighbors((2, 2), (5, 5)).collect::<Vec<_>>().len(), 8);
        assert_eq!(neighbors((4, 4), (5, 5)).collect::<Vec<_>>().len(), 3);
        assert_eq!(neighbors((3, 4), (5, 5)).collect::<Vec<_>>().len(), 5);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    On,
    Off,
}

fn part1(input: &str) -> usize {
    let mut grid = parse(&input);

    for _ in 0..100 {
        grid = step(&mut grid, false);
    }

    count(&grid)
}

fn part2(input: &str) -> usize {
    let mut grid = parse(&input);

    for _ in 0..100 {
        grid = step(&mut grid, true);
    }

    count(&grid)
}

fn parse(input: &str) -> Vec<Vec<State>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { State::On } else { State::Off })
                .collect()
        })
        .collect()
}

fn step(grid: &mut Vec<Vec<State>>, corners: bool) -> Vec<Vec<State>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut new_grid = vec![vec![State::Off; m]; n];

    if corners {
        grid[0][0] = State::On;
        grid[n - 1][0] = State::On;
        grid[0][m - 1] = State::On;
        grid[n - 1][m - 1] = State::On;
    }

    for i in 0..n {
        for j in 0..m {
            let mut count = 0;
            for (x, y) in neighbors((i, j), (n, m)) {
                if let State::On = grid[x][y] {
                    count += 1;
                }
            }

            new_grid[i][j] = match grid[i][j] {
                State::On => {
                    if count == 2 || count == 3 {
                        State::On
                    } else {
                        State::Off
                    }
                }
                State::Off => {
                    if count == 3 {
                        State::On
                    } else {
                        State::Off
                    }
                }
            }
        }
    }

    if corners {
        new_grid[0][0] = State::On;
        new_grid[n - 1][0] = State::On;
        new_grid[0][m - 1] = State::On;
        new_grid[n - 1][m - 1] = State::On;
    }
    new_grid
}

fn neighbors(
    (x, y): (usize, usize),
    (x_max, y_max): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    (-1..2)
        .flat_map(move |i: i64| (-1..2).map(move |j: i64| (i, j)))
        .filter_map(move |(dx, dy)| {
            let x_new = x as i64 + dx;
            let y_new = y as i64 + dy;

            if x_new >= 0
                && y_new >= 0
                && x_new < x_max as i64
                && y_new < y_max as i64
                && (dx != 0 || dy != 0)
            {
                Some((x_new as usize, y_new as usize))
            } else {
                None
            }
        })
}

fn count(grid: &Vec<Vec<State>>) -> usize {
    grid.into_iter()
        .map(|vec| {
            vec.into_iter()
                .map(|&state| if state == State::On { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}
