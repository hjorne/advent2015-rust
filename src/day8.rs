use std::fs;

pub fn day8() {
    let input = fs::read_to_string("inputs/day8.txt").unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();

    let total: usize = input.iter().map(|s| s.len()).sum();

    part1(&input, total);
    part2(&input, total);
}

fn part1(input: &Vec<&str>, total: usize) {
    let reduced_total: usize = input.into_iter().map(|s| filtered_str(s).len()).sum();

    println!("The answer is {}", total - reduced_total);
}

fn part2(input: &Vec<&str>, total: usize) {
    let expanded_total: usize = input.into_iter().map(|s| expanded_str(s).len()).sum();

    println!("The answer is {}", expanded_total - total);
}

fn filtered_str(s: &str) -> String {
    let mut new_s = String::new();
    let mut i = 1;
    let str_arr: Vec<_> = s.chars().collect();

    while i < str_arr.len() - 1 {
        if str_arr[i] == '\\' {
            if str_arr[i + 1] == '\\' {
                new_s.push('\\');
                i += 2;
            } else if str_arr[i + 1] == '"' {
                new_s.push('"');
                i += 2;
            } else if str_arr[i + 1] == 'x'
                && str_arr[i + 2].is_ascii_hexdigit()
                && str_arr[i + 3].is_ascii_hexdigit()
            {
                new_s.push('x');
                i += 4;
            }
        } else {
            new_s.push(str_arr[i]);
            i += 1;
        }
    }

    new_s
}

fn expanded_str(s: &str) -> String {
    let mut new_s = String::new();
    new_s.push('"');

    for c in s.chars() {
        if c == '\\' {
            new_s.push_str("\\\\");
        } else if c == '"' {
            new_s.push_str("\\\"");
        } else {
            new_s.push(c);
        }
    }

    new_s.push('"');
    new_s
}
