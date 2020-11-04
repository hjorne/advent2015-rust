#![allow(dead_code)]

use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type E = Box<dyn std::error::Error>;

const MAX_T: i64 = 2503;

#[derive(Debug, Eq, PartialEq)]
struct Reindeer<'a> {
    name: &'a str,
    speed: i64,
    endurance: i64,
    rest: i64,
}

#[derive(Debug, PartialEq, Eq)]
enum Event<'a> {
    Start(&'a Reindeer<'a>, i64, i64),
    End(&'a Reindeer<'a>, i64, i64),
}

impl<'a> Event<'a> {
    fn get_t(&self) -> i64 {
        match self {
            Event::Start(_, _, t) => *t,
            Event::End(_, _, t) => *t,
        }
    }
}

impl<'a> PartialOrd for Event<'a> {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Event<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_t().cmp(&other.get_t()).reverse()
    }
}

struct Timeline<'a> {
    timeline: RefCell<BinaryHeap<Event<'a>>>,
}

impl<'a> Timeline<'a> {
    fn new() -> Timeline<'a> {
        Timeline {
            timeline: RefCell::new(BinaryHeap::new()),
        }
    }

    fn add(&self, event: Event<'a>) {
        self.timeline.borrow_mut().push(event);
    }

    fn pop(&self) -> Option<Event> {
        self.timeline.borrow_mut().pop()
    }
}

#[derive(Debug)]
struct History<'a> {
    history: HashMap<i64, Vec<(&'a str, i64)>>,
}

impl<'a> History<'a> {
    fn new() -> History<'a> {
        History {
            history: HashMap::new(),
        }
    }

    fn add(&mut self, name: &'a str, t: i64, x: i64) {
        self.history
            .entry(t)
            .or_insert(vec![(name, x)])
            .push((name, x))
    }

    fn score(&self) -> i64 {
        let mut counter: HashMap<&str, i64> = HashMap::new();
        for (_, vec) in &self.history {
            let (_, max_val) = vec
                .into_iter()
                .max_by(|&(_, x), &(_, y)| x.cmp(&y))
                .unwrap();

            vec.into_iter()
                .filter(|(_, d)| d == max_val)
                .map(|(name, _)| name)
                .collect::<HashSet<_>>()
                .into_iter()
                .for_each(|name| *counter.entry(name).or_insert(0) += 1)
        }

        *counter.values().max().unwrap() - 1
    }
}

fn run(reindeers: &Vec<Reindeer>) -> (i64, i64) {
    let mut max = 0;
    let mut history = History::new();
    let timeline = Timeline::new();
    for reindeer in reindeers {
        timeline.add(Event::Start(reindeer, 0, 0));
    }

    while let Some(event) = timeline.pop() {
        if event.get_t() == MAX_T {
            continue;
        }

        let new_event = match event {
            Event::Start(reindeer, x, t) => {
                let (new_x, new_t) = calculate_new_pos(x, t, reindeer.speed, reindeer.endurance);
                interpolate_history(&mut history, reindeer.name, x, t, new_t, reindeer.speed);
                Event::End(reindeer, new_x, new_t)
            }
            Event::End(reindeer, x, t) => {
                if x > max {
                    max = x;
                }
                let (new_x, new_t) = calculate_new_pos(x, t, 0, reindeer.rest);
                interpolate_history(&mut history, reindeer.name, x, t, new_t, 0);
                Event::Start(reindeer, new_x, new_t)
            }
        };

        timeline.add(new_event);
    }

    let score = history.score();
    (max, score)
}

fn calculate_new_pos(x: i64, t: i64, v: i64, dt: i64) -> (i64, i64) {
    let dt = if t + dt > MAX_T { MAX_T - t } else { dt };

    (x + v * dt, t + dt)
}

fn interpolate_history<'a>(
    history: &mut History<'a>,
    name: &'a str,
    x: i64,
    t1: i64,
    t2: i64,
    v: i64,
) {
    for t_inner in t1..t2 + 1 {
        history.add(name, t_inner, x + (t_inner - t1) * v);
    }
}

fn parse(input: &str) -> Vec<Reindeer> {
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    re.captures_iter(input)
        .map(|capture| {
            let name = capture.get(1).unwrap().as_str();
            let speed = capture.get(2).unwrap().as_str().parse().unwrap();
            let endurance = capture.get(3).unwrap().as_str().parse().unwrap();
            let rest = capture.get(4).unwrap().as_str().parse().unwrap();

            Reindeer {
                name,
                speed,
                endurance,
                rest,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn day14_test() -> Result<(), E> {
        let input = fs::read_to_string("inputs/day14.txt")?;
        let reindeers = parse(&input);
        assert_eq!(reindeers.len(), 9);

        let (part1, part2) = run(&reindeers);
        assert_eq!(part1, 2660);
        assert_eq!(part2, 1256);
        Ok(())
    }
}
