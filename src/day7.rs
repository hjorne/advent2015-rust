use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
enum Op<'a> {
    Not(Value<'a>),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    Lshift(&'a str, u32),
    Rshift(&'a str, u32),
    Assign(Value<'a>),
}

#[derive(Debug, Copy, Clone)]
enum Value<'a> {
    Node(&'a str),
    Explicit(u32),
}

pub fn day7() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    let (g, mut operations_map) = parse(&input);

    let a_wire = run(&g, &operations_map);
    operations_map.insert("b", Op::Assign(Value::Explicit(a_wire)));
    run(&g, &operations_map);
}

fn run(g: &DiGraphMap<&str, ()>, operations_map: &HashMap<&str, Op>) -> u32 {
    let mut value_map = HashMap::new();

    toposort(&g, None)
        .unwrap()
        .into_iter()
        .for_each(|node: &str| {
            match operations_map.get(node).expect("Fucked up") {
                Op::Not(u) => value_map.insert(node, !read_value(u, &value_map)),
                Op::Lshift(u, val) => value_map.insert(node, value_map.get(u).unwrap() << val),
                Op::Rshift(u, val) => value_map.insert(node, value_map.get(u).unwrap() >> val),
                Op::Assign(u) => value_map.insert(node, read_value(u, &value_map)),
                Op::And(u, v) => {
                    value_map.insert(node, read_value(u, &value_map) & read_value(v, &value_map))
                }
                Op::Or(u, v) => {
                    value_map.insert(node, read_value(u, &value_map) | read_value(v, &value_map))
                }
            };
        });

    let a_wire = value_map.get("a").unwrap();
    println!("Wire a's value is {}", a_wire);
    *a_wire
}

fn read_value(value: &Value, value_map: &HashMap<&str, u32>) -> u32 {
    match value {
        Value::Node(node) => *value_map.get(node).expect("No value found in value_map"),
        Value::Explicit(val) => *val,
    }
}

fn parse(input: &str) -> (DiGraphMap<&str, ()>, HashMap<&str, Op>) {
    let mut operations_map = HashMap::new();
    let mut edges = Vec::new();

    input.lines().for_each(|line| {
        let split: Vec<_> = line.split(" -> ").collect();
        let dest = split[1];
        let split: Vec<_> = line.split_whitespace().collect();

        if line.contains("NOT") {
            let v = write_value(split[1]);
            operations_map.insert(dest, Op::Not(v));
            if let Value::Node(node) = v {
                edges.push((node, dest));
            }
        } else if line.contains("OR") {
            let v1 = write_value(split[0]);
            let v2 = write_value(split[2]);
            operations_map.insert(dest, Op::Or(v1, v2));
            if let Value::Node(node) = v1 {
                edges.push((node, dest));
            }
            if let Value::Node(node) = v2 {
                edges.push((node, dest));
            }
        } else if line.contains("AND") {
            let v1 = write_value(split[0]);
            let v2 = write_value(split[2]);
            operations_map.insert(dest, Op::And(v1, v2));
            if let Value::Node(node) = v1 {
                edges.push((node, dest));
            }
            if let Value::Node(node) = v2 {
                edges.push((node, dest));
            }
        } else if line.contains("LSHIFT") {
            edges.push((split[0], dest));
            operations_map.insert(dest, Op::Lshift(split[0], split[2].parse().unwrap()));
        } else if line.contains("RSHIFT") {
            edges.push((split[0], dest));
            operations_map.insert(dest, Op::Rshift(split[0], split[2].parse().unwrap()));
        } else {
            let v = write_value(split[0]);
            operations_map.insert(dest, Op::Assign(v));
            if let Value::Node(node) = v {
                edges.push((node, dest));
            }
        }
    });

    (DiGraphMap::from_edges(&edges), operations_map)
}

fn write_value(s: &str) -> Value {
    let parse = s.parse::<u32>();
    if parse.is_ok() {
        Value::Explicit(parse.unwrap())
    } else {
        Value::Node(s)
    }
}
