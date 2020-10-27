use std::fs;

static VOWELS: &str = "aeiou";
static BAD_STR: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn day5() {
    let input = fs::read_to_string("inputs/day5.txt") .expect("Couldn't read from file");
    let input: Vec<&str> = input.split_whitespace().collect();

    nice_counter(&input, is_nice1);
    nice_counter(&input, is_nice2);
}

fn nice_counter(input: &Vec<&str>, f: fn(&str) -> bool) {
    let nice_count = input.into_iter().filter(|&s| f(s)).count();
    println!("Number of nice characters is {}", nice_count);
}

fn is_nice1(s: &str) -> bool {
    let vowel_count = s.chars().filter(|&c| VOWELS.contains(c)).count() >= 3;
    let double_letter = s.chars().zip(s[1..].chars()).any(|(c1, c2)| c1 == c2);
    let bad_str = s.chars().zip(s[1..].chars()).any(|(c1, c2)| BAD_STR.contains(&c2s(c1, c2).as_str()));
    return !bad_str && vowel_count && double_letter
}

fn is_nice2(s: &str) -> bool {
    let s_arr = s.as_bytes();

    let repeat_adj = {
        let mut repeat_adj = false;
        for i in 0..s.len()- 2 {
            if s_arr[i] == s_arr[i+2] {
                repeat_adj = true;
                break;
            }
        }
        repeat_adj
    };

    let appears_twice = {
        let mut appears_twice = false;
        for i in 0..s_arr.len()-1 {
            for j in i+2..s_arr.len()-1 {
                if s_arr[i] == s_arr[j] && s_arr[i+1] == s_arr[j+1] {
                    appears_twice = true;
                    break;
                }
            }
        }
        appears_twice
    };

    repeat_adj && appears_twice
}

fn c2s(c1: char, c2: char) -> String {
    let mut s = String::from(c1);
    s.push(c2);
    s
}
