use serde_json::Value;
use std::error::Error;
use std::fs;

type E = Box<dyn Error>;

#[allow(dead_code)]
pub fn day12() -> Result<(), E> {
    let input = fs::read_to_string("inputs/day12.txt")?;
    let json: Value = serde_json::from_str(&input)?;

    println!("The answer is {}", sum(&json));

    Ok(())
}

fn sum(json: &Value) -> i64 {
    match json {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(arr) => arr.into_iter().map(|value| sum(value)).sum(),
        Value::Object(map) => {
            if map.values().any(|value| value == "red") {
                0
            } else {
                map.values().into_iter().map(|value| sum(value)).sum()
            }
        }
    }
}

#[test]
fn day12_test() {
    day12().unwrap();
}
