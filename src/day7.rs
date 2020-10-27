use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Not(String),
    And(String, String),
    Or(String, String),
    Lshift(String, u32),
    Rshift(String, u32),
    Value(u32),
    Assign(String)
}


pub fn day7() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    println!("{:?}", parse(&input));
}

fn parse(input: &str) -> HashMap<String, Op> {
    input.lines().map(|line| {
        let split: Vec<_> = line.split(" -> ").collect();
        let dest: String = split[1].to_owned();
        let op = parse_op(split[0]);
        (dest, op)
    }).collect()
}

fn parse_op(line: &str) -> Op {
    let split: Vec<_> = line.split_whitespace().collect();
    if line.contains("NOT") {
        Op::Not(split[1].to_owned())
    } else if line.contains("OR") {
        Op::Or(split[0].to_owned(), split[2].to_owned())
    } else if line.contains("AND") {
        Op::And(split[0].to_owned(), split[2].to_owned())
    } else if line.contains("LSHIFT") {
        Op::Lshift(split[0].to_owned(), split[2].parse().unwrap())
    } else if line.contains("RSHIFT") {
        Op::Rshift(split[0].to_owned(), split[2].parse().unwrap())
    } else {
        let parse = split[0].parse::<u32>();
        if parse.is_ok(){
            Op::Value(parse.unwrap())
        } else {
            Op::Assign(split[0].to_owned())
        }
    }
}
