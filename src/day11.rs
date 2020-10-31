pub fn day11() {
    let input = "cqjxjnds";
    let mut input = input.chars().collect::<Vec<_>>();

    while !legal(&input) {
        increment_str(&mut input);
    }

    println!("{:?}", input.iter().collect::<String>());
    increment_str(&mut input);

    while !legal(&input) {
        increment_str(&mut input);
    }
    println!("{:?}", input.iter().collect::<String>());
}

fn increment_str(chars: &mut [char]) {
    let len = chars.len();
    if len == 0 {
        return;
    }

    if chars[len - 1] == 'z' {
        chars[len - 1] = 'a';
        increment_str(&mut chars[0..len - 1])
    } else {
        chars[len - 1] = increment_char(chars[len - 1]);
    }
}

fn increment_char(c: char) -> char {
    (c as u8 + 1) as char
}

fn legal(chars: &[char]) -> bool {
    straight(chars) && no_illegal_letters(chars) && non_overlapping_pairs(chars)
}

fn straight(chars: &[char]) -> bool {
    for i in 0..chars.len() - 3 {
        if chars[i] as u8 + 1 == chars[i + 1] as u8 && chars[i + 1] as u8 + 1 == chars[i + 2] as u8
        {
            return true;
        }
    }
    return false;
}

fn no_illegal_letters(chars: &[char]) -> bool {
    !chars.contains(&'i') & !chars.contains(&'o') && !chars.contains(&'l')
}

fn non_overlapping_pairs(chars: &[char]) -> bool {
    let mut i = 0;
    let mut count = 0;
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            count += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    count >= 2
}
