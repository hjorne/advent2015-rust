pub fn day10() {
    let input = "1321131112";

    let iter40 = iter_look_and_say(input, 40);
    let iter50 = iter_look_and_say(input, 50);

    println!("Look and say after 40 is of size {}", iter40.len());
    println!("Look and say after 50 is of size {}", iter50.len());
}

fn look_and_say(input: &str) -> String {
    let digits = input.chars().collect::<Vec<_>>();
    let mut target = digits[0];
    let mut count = 1;
    let mut result = String::new();

    for i in 1..digits.len() {
        if digits[i] == target {
            count += 1;
        } else {
            result.push_str(&format!("{}{}", count, target));
            count = 1;
            target = digits[i];
        }
    }

    result.push_str(&format!("{}{}", count, target));

    result
}

fn iter_look_and_say(input: &str, count: u32) -> String {
    let mut result = input.to_owned();
    for _ in 0..count {
        result = look_and_say(&result);
    }

    result
}
